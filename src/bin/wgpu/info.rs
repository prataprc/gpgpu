use colored::Colorize;
use prettytable::cell;
use winit::{
    event_loop::EventLoop,
    monitor::{MonitorHandle, VideoMode},
    window::WindowBuilder,
};

use gpgpu::{
    err_at,
    util::{self, PrettyRow},
    wg, Error, Result,
};

use crate::Opt;

pub fn info_global_report(opts: &Opt) -> Result<()> {
    let inst = wgpu::Instance::new(wg::backend().into());
    let gr = inst.generate_report();
    let mut srs: Vec<wg::pretty::StorageReport> = vec![("surfaces", gr.surfaces).into()];

    let mut extend_hub_report = |hr: &wgpu_core::hub::HubReport| {
        srs.extend_from_slice(&vec![
            ("adapters", hr.adapters.clone()).into(),
            ("devices", hr.devices.clone()).into(),
            ("pipeline_layouts", hr.pipeline_layouts.clone()).into(),
            ("shader_modules", hr.shader_modules.clone()).into(),
            ("bind_group_layouts", hr.bind_group_layouts.clone()).into(),
            ("bind_groups", hr.bind_groups.clone()).into(),
            ("command_buffers", hr.command_buffers.clone()).into(),
            ("render_bundles", hr.render_bundles.clone()).into(),
            ("render_pipelines", hr.render_pipelines.clone()).into(),
            ("compute_pipelines", hr.compute_pipelines.clone()).into(),
            ("query_sets", hr.query_sets.clone()).into(),
            ("buffers", hr.buffers.clone()).into(),
            ("textures", hr.textures.clone()).into(),
            ("texture_views", hr.texture_views.clone()).into(),
            ("samplers", hr.samplers.clone()).into(),
        ]);
    };

    #[cfg(target_os = "linux")]
    gr.vulkan.as_ref().map(|hr| extend_hub_report(hr));
    #[cfg(target_os = "macos")]
    gr.metal.as_ref().map(|hr| extend_hub_report(hr));

    util::make_table(&srs).print_tty(!opts.no_color);
    Ok(())
}

// List monitors or show video modes for primary monitor or monitor chosen by index `n`.
pub fn info_window(
    modes: bool,
    n: Option<usize>,
    opts: &Opt,
    config: &wg::Config,
) -> Result<()> {
    let eloop = EventLoop::new();
    let window = {
        let mut wb = WindowBuilder::new();
        wb.window = config.to_window_attributes()?;
        err_at!(Fatal, wb.build(&eloop))?
    };

    println!(
        " Primary monitor: {:?}",
        window.current_monitor().map(|m| m.name())
    );
    println!(
        " Current monitor: {:?}",
        window.current_monitor().map(|m| m.name())
    );
    println!();

    let monitors: Vec<MonitorHandle> = window.available_monitors().collect();

    match n {
        Some(n) if modes => {
            // show video modes for monitor index `n`
            let modes = monitors[n].video_modes().collect::<Vec<VideoMode>>();
            util::make_table(&modes).print_tty(!opts.no_color);
        }
        None if modes => match window.primary_monitor() {
            Some(primary) => {
                // show video modes for primary monitor.
                let modes = primary.video_modes().collect::<Vec<VideoMode>>();
                util::make_table(&modes).print_tty(!opts.no_color);
            }
            None => println!("{}", "No primary monitor".red()),
        },
        _ => {
            match window.primary_monitor() {
                Some(primary) => {
                    util::make_table(&vec![primary]).print_tty(!opts.no_color);
                }
                None => println!("{}", "No primary monitor".red()),
            }
            println!();
            util::make_table(&monitors).print_tty(!opts.no_color);
        }
    }

    Ok(())
}

pub fn info_adapters(opts: &Opt) -> Result<()> {
    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let adapters: Vec<wgpu::Adapter> =
        instance.enumerate_adapters(wgpu::Backends::all()).collect();

    let infos: Vec<wgpu::AdapterInfo> = adapters.iter().map(|a| a.get_info()).collect();
    util::make_table(&infos).print_tty(opts.no_color);

    Ok(())
}

pub fn info_features(opts: &Opt) -> Result<()> {
    let mut features = wg::pretty::features();

    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let adapters: Vec<wgpu::Adapter> =
        instance.enumerate_adapters(wgpu::Backends::all()).collect();

    adapters
        .iter()
        .for_each(|a| wg::pretty::add_adapter_to_features(&mut features, a.features()));

    let mut table = prettytable::Table::new();
    let table = match features.len() {
        0 => table,
        _ => {
            let mut titles = wg::pretty::Feature::to_head();
            for a in adapters.iter() {
                let mut name = a.get_info().name.clone();
                name.truncate(10);
                titles.add_cell(cell![Fy -> name]);
            }
            table.set_titles(titles);

            features.iter().for_each(|r| {
                table.add_row(r.to_row());
            });
            table.set_format(wg::pretty::Feature::to_format());
            table
        }
    };

    table.print_tty(opts.no_color);

    Ok(())
}

pub fn info_limits(opts: &Opt) -> Result<()> {
    let mut limits = wg::pretty::limits();

    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let adapters: Vec<wgpu::Adapter> =
        instance.enumerate_adapters(wgpu::Backends::all()).collect();

    adapters
        .iter()
        .for_each(|a| wg::pretty::add_adapter_to_limits(&mut limits, a.limits()));

    let mut table = prettytable::Table::new();
    let table = match limits.len() {
        0 => table,
        _ => {
            let mut titles = wg::pretty::Limit::to_head();
            for a in adapters.iter() {
                let mut name = a.get_info().name.clone();
                name.truncate(10);
                titles.add_cell(cell![Fy -> name]);
            }
            table.set_titles(titles);

            limits.iter().for_each(|r| {
                table.add_row(r.to_row());
            });
            table.set_format(wg::pretty::Limit::to_format());
            table
        }
    };

    table.print_tty(opts.no_color);

    Ok(())
}

pub fn info_texture_formats(opts: &Opt) -> Result<()> {
    let info = wg::pretty::texture_formats_info();
    util::make_table(&info).print_tty(opts.no_color);

    Ok(())
}
