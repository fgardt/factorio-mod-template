use rivets::detour;
use rivets::Opaque;

#[detour(?valid@LuaSurface@@UEBA_NXZ)]
fn valid(this: Opaque) -> bool {
    println!("Hello from LuaSurface::valid!");
    unsafe { back(this) }
}