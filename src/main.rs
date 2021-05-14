mod api;
mod overlay;

use api::APIHandle;
use overlay::Overlay;

fn main() {
    //Create a handle to the process
    let process_handle = APIHandle::new().expect("Could not create handle to target process.");

    let overlay = Overlay::new((500, 500));

    overlay.run(process_handle);
}
