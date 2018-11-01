mod slice;

use tfdeploy::ops as tfdops;
use tfdeploy::ops::prelude::*;

use ops::OpRegister;
use pb::NodeProto;

pub fn register_all_ops(reg: &mut OpRegister) {
    reg.insert("Concat", concat);
    reg.insert("Expand", |_| {
        Ok(Box::new(tfdops::array::MultiBroadcastTo::default()))
    });
    reg.insert("Flatten", flatten);
    reg.insert("Pad", pad);
    reg.insert("Reshape", |_| {
        Ok(Box::new(tfdops::array::Reshape::default()))
    });
    reg.insert("Shape", |_| {
        Ok(Box::new(tfdops::array::Shape::new(DatumType::I64)))
    });
    reg.insert("Size", |_| {
        Ok(Box::new(tfdops::array::Size::new(DatumType::I64)))
    });
    reg.insert("Transpose", transpose);
    reg.insert("Slice", slice);
    reg.insert("Split", split);
    reg.insert("Squeeze", squeeze);
    reg.insert("Unsqueeze", unsqueeze);
}

pub fn concat(node: &NodeProto) -> TfdResult<Box<Op>> {
    let axis = node.get_attr_int("axis")?;
    Ok(Box::new(tfdops::array::Concat::new(axis as usize)))
}

pub fn flatten(node: &NodeProto) -> TfdResult<Box<Op>> {
    let axis = node.get_attr_opt_int("axis")?.unwrap_or(1);
    Ok(Box::new(tfdops::array::Flatten::new(axis as usize)))
}

pub fn pad(node: &NodeProto) -> TfdResult<Box<Op>> {
    let mode = node.get_attr_opt_str("mode")?;
    let value = node.get_attr_opt_float("value")?;
    let mode = match mode {
        Some("reflect") => tfdops::array::PadMode::Reflect,
        Some("edge") => tfdops::array::PadMode::Edge,
        _ => tfdops::array::PadMode::Constant(value.unwrap_or(0.0)),
    };
    let pads = node.get_attr_ints("pads")?;
    let rank = pads.len()/2;
    let pads = (0..rank).map(|ax| (pads[ax] as usize, pads[ax+rank] as usize)).collect();
    Ok(Box::new(tfdops::array::Pad::new(pads, mode)))
}


pub fn slice(node: &NodeProto) -> TfdResult<Box<Op>> {
    let axes = node.get_attr_opt_ints("axes")?;
    let begin = node.get_attr_ints("starts")?;
    let end = node.get_attr_ints("ends")?;
    Ok(Box::new(slice::Slice::new(
        axes.map(|a| a.into_iter().map(|&d| d as _).collect()),
        begin.iter().map(|&d| d as _).collect(),
        end.iter().map(|&d| d as _).collect(),
    )))
}

pub fn split(node: &NodeProto) -> TfdResult<Box<Op>> {
    let axis = node.get_attr_opt_int("axis")?.unwrap_or(0);
    let split = node.get_attr_opt_ints("split")?;
    Ok(Box::new(tfdops::array::Split::new(
        axis as usize,
        node.get_output().len(),
        split.map(|a| a.into_iter().map(|&d| d as _).collect()),
    )))
}

pub fn squeeze(node: &NodeProto) -> TfdResult<Box<Op>> {
    let axes = node
        .get_attr_opt_ints("axes")?
        .map(|l| l.iter().map(|&a| a as usize).collect());
    Ok(Box::new(tfdops::array::Squeeze::new(axes)))
}

pub fn transpose(node: &NodeProto) -> TfdResult<Box<Op>> {
    let perm = node
        .get_attr_opt_ints("perm")?
        .map(|axes| axes.iter().map(|&a| a as usize).collect());
    Ok(Box::new(tfdops::array::PermuteAxes::new(perm)))
}

pub fn unsqueeze(node: &NodeProto) -> TfdResult<Box<Op>> {
    let axes = node
        .get_attr_ints("axes")?
        .iter()
        .map(|&a| a as usize)
        .collect();
    Ok(Box::new(tfdops::array::AddDims::new(axes)))
}
