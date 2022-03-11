use structopt::StructOpt;

use cgi::{util, wg, Error, Result};

#[derive(StructOpt)]
pub struct Opt {
    #[structopt(long = "debug")]
    debug: bool,

    #[structopt(long = "no-color")]
    no_color: bool,

    #[structopt(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clone, StructOpt)]
pub enum SubCommand {
    Backend,
    Report,
}

fn main() {
    let opts = Opt::from_args();

    let res = match &opts.subcmd {
        SubCommand::Report => info_global_report(&opts),
        SubCommand::Backend => {
            println!("{:?} backend is used", wg::backend());
            Ok(())
        }
    };

    res.map_err(|err: Error| println!("unexpected error: {}", err))
        .ok();
}

fn info_global_report(opts: &Opt) -> Result<()> {
    let inst = wgpu::Instance::new(wg::backend());
    let gr = inst.generate_report();
    let mut srs = vec![gr.surfaces];

    let mut extend_hub_report = |hr: &wgpu_core::hub::HubReport| {
        srs.extend_from_slice(&vec![
            hr.adapters.clone(),
            hr.devices.clone(),
            hr.pipeline_layouts.clone(),
            hr.shader_modules.clone(),
            hr.bind_group_layouts.clone(),
            hr.bind_groups.clone(),
            hr.command_buffers.clone(),
            hr.render_bundles.clone(),
            hr.render_pipelines.clone(),
            hr.compute_pipelines.clone(),
            hr.query_sets.clone(),
            hr.buffers.clone(),
            hr.textures.clone(),
            hr.texture_views.clone(),
            hr.samplers.clone(),
        ]);
    };

    #[cfg(target_os = "linux")]
    gr.vulkan.as_ref().map(|hr| extend_hub_report(hr));
    #[cfg(target_os = "macos")]
    gr.metal.as_ref().map(|hr| extend_hub_report(hr));

    util::make_table(&srs).print_tty(!opts.no_color);
    Ok(())
}
