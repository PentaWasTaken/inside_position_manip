mod api;
mod overlay;

use std::borrow::Borrow;

use api::APIHandle;
use overlay::Overlay;

use std::convert::TryInto;

fn main() {
    //Create a handle to the process
    let api_handle = APIHandle::new().expect("Could not create handle to target process.");

    let overlay = Overlay::new((500, 300));

    overlay.run(api_handle);
}
