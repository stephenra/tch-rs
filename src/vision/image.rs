use crate::utils::{path_to_str, TorchError};
use crate::Tensor;
use libc::c_int;

/// On success returns a tensor of shape [width, height, channels].
fn load_hwc(path: &std::path::Path) -> Result<Tensor, TorchError> {
    let path = std::ffi::CString::new(path_to_str(path)?)?;
    let c_tensor = unsafe_torch_err!({ torch_sys::at_load_image(path.as_ptr()) });
    Ok(Tensor { c_tensor })
}

/// Expects a tensor of shape [width, height, channels].
fn save_hwc(t: &Tensor, path: &std::path::Path) -> Result<(), TorchError> {
    let path = std::ffi::CString::new(path_to_str(path)?)?;
    let _ = unsafe_torch_err!({ torch_sys::at_save_image(t.c_tensor, path.as_ptr()) });
    Ok(())
}

/// Expects a tensor of shape [width, height, channels].
/// On success returns a tensor of shape [width, height, channels].
fn resize_hwc(t: &Tensor, out_w: i64, out_h: i64) -> Tensor {
    let c_tensor = unsafe_torch!({ torch_sys::at_resize_image(t.c_tensor, out_w as c_int, out_h as c_int) });
    Tensor { c_tensor }
}

fn hwc_to_chw(tensor: &Tensor) -> Tensor {
    tensor.permute(&[2, 0, 1])
}

fn chw_to_hwc(tensor: &Tensor) -> Tensor {
    tensor.permute(&[1, 2, 0])
}

pub fn load(path: &std::path::Path) -> Result<Tensor, TorchError> {
    let tensor = load_hwc(path)?;
    Ok(hwc_to_chw(&tensor))
}

pub fn save(t: &Tensor, path: &std::path::Path) -> Result<(), TorchError> {
    save_hwc(&chw_to_hwc(t), path)
}

pub fn resize(t: &Tensor, out_w: i64, out_h: i64) -> Tensor {
    hwc_to_chw(&resize_hwc(&chw_to_hwc(t), out_w, out_h))
}