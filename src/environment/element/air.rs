use crate::colors::BLANK;

use crate::environment::element::{Element, ElementData, ElementKind};

pub struct Air {}
impl ElementData for Air {
	fn new() -> Element {
		Element {
			kind: ElementKind::Air,
			color: BLANK
		}
	}

	fn tick(elem: &Element, world: &mut crate::environment::world::World) {
		println!("{:?} {:?} ticked", elem.color, elem.kind);
	}
}