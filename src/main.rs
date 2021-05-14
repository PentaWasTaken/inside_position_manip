mod api;
mod overlay;

use api::APIHandle;
use overlay::Overlay;

fn main() {
    //Create a handle to the process
    let process_handle = APIHandle::new().unwrap();

    let overlay = Overlay::new();

    overlay.run(process_handle);
}
