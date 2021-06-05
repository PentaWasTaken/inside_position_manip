mod api;
mod input_handler;
mod overlay;

use api::APIHandle;
use overlay::Overlay;

fn main() {
    //Create a handle to the process
    let api_handle = APIHandle::new().expect("Could not create handle to target process.");

    let overlay = Overlay::new((500, 300));

    overlay.run(api_handle);
}
