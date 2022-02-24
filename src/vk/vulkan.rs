use crate::{Error, Result};

pub fn layer_properties() -> Result<Vec<vulkano::instance::LayerProperties>> {
    use vulkano::instance::layers_list;

    let mut layers = vec![];
    for layer in err_at!(Fatal, layers_list())? {
        layers.push(layer)
    }

    Ok(layers)
}

pub fn layer_names() -> Result<Vec<String>> {
    Ok(layer_properties()?
        .into_iter()
        .map(|l| l.name().to_string())
        .collect())
}

pub fn check_layer_names(layers: Vec<String>) -> Result<Vec<String>> {
    let available = layer_names()?;
    Ok(layers
        .into_iter()
        .filter(|l| available.contains(l))
        .collect())
}
