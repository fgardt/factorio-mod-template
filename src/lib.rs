use rivets::detour;
use rivets::Opaque;

#[detour(?valid@LuaSurface@@UEBA_NXZ)]
const fn valid(this: Opaque) -> bool {
    println!("Hello from LuaSurface::valid!");
    unsafe { back(this) }
}

rivets::initialize!();