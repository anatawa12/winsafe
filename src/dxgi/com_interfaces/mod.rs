mod idxgiadapter;
mod idxgidevice;
mod idxgidevicesubobject;
mod idxgifactory;
mod idxgiobject;
mod idxgioutput;
mod idxgisurface;

pub mod decl {
	pub use super::idxgiadapter::IDXGIAdapter;
	pub use super::idxgidevice::IDXGIDevice;
	pub use super::idxgidevicesubobject::IDXGIDeviceSubObject;
	pub use super::idxgifactory::IDXGIFactory;
	pub use super::idxgiobject::IDXGIObject;
	pub use super::idxgioutput::IDXGIOutput;
	pub use super::idxgisurface::IDXGISurface;
}

pub mod traits {
	pub use super::idxgiadapter::dxgi_IDXGIAdapter;
	pub use super::idxgidevice::dxgi_IDXGIDevice;
	pub use super::idxgidevicesubobject::dxgi_IDXGIDeviceSubObject;
	pub use super::idxgifactory::dxgi_IDXGIFactory;
	pub use super::idxgiobject::dxgi_IDXGIObject;
	pub use super::idxgioutput::dxgi_IDXGIOutput;
	pub use super::idxgisurface::dxgi_IDXGISurface;
}

pub mod vt {
	pub use super::idxgiadapter::IDXGIAdapterVT;
	pub use super::idxgidevice::IDXGIDeviceVT;
	pub use super::idxgidevicesubobject::IDXGIDeviceSubObjectVT;
	pub use super::idxgifactory::IDXGIFactoryVT;
	pub use super::idxgiobject::IDXGIObjectVT;
	pub use super::idxgioutput::IDXGIOutputVT;
	pub use super::idxgisurface::IDXGISurfaceVT;
}
