#[derive(Clone)]
#[derive(Copy)]
enum Presence {
  Yes(u16),
  No(),
}
impl Presence {
  pub fn new(pres:bool, id:u16) -> Presence {
    if pres {
      return Presence::Yes(id)
    }
    Presence::No()
  }
  pub fn something(&self) -> (bool, u16) {
    match self {
      Presence::Yes(u) => (true, *u),
      Presence::No() => (false, 0),
    }
  }
}
