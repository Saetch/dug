use crate::drawable_object::drawable_object::DrawableObject;

pub(crate) trait LogicObject {
}

pub(crate) trait GameObject: LogicObject+DrawableObject {}