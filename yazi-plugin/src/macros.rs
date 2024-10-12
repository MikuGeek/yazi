#[macro_export]
macro_rules! impl_style_method {
	($methods:ident, $($field:tt).+) => {
		$methods.add_function("style", |_, (ud, value): (mlua::AnyUserData, mlua::Value)| {
			ud.borrow_mut::<Self>()?.$($field).+ = $crate::elements::Style::try_from(value)?.0;
			Ok(ud)
		});
	};
}

#[macro_export]
macro_rules! impl_area_method {
	($methods:ident) => {
		use mlua::IntoLua;
		use $crate::elements::Rect;

		$methods.add_function("area", |lua, (ud, area): (mlua::AnyUserData, Option<Rect>)| {
			if let Some(r) = area {
				ud.borrow_mut::<Self>()?.area = r;
				ud.into_lua(lua)
			} else {
				ud.borrow::<Self>()?.area.into_lua(lua)
			}
		});
	};
}

#[macro_export]
macro_rules! impl_style_shorthands {
	($methods:ident, $($field:tt).+) => {
		$methods.add_function("fg", |_, (ud, color): (mlua::AnyUserData, String)| {
			ud.borrow_mut::<Self>()?.$($field).+.fg = yazi_shared::theme::Color::try_from(color).ok().map(Into::into);
			Ok(ud)
		});
		$methods.add_function("bg", |_, (ud, color): (mlua::AnyUserData, String)| {
			ud.borrow_mut::<Self>()?.$($field).+.bg = yazi_shared::theme::Color::try_from(color).ok().map(Into::into);
			Ok(ud)
		});
		$methods.add_function("bold", |_, ud: mlua::AnyUserData| {
			ud.borrow_mut::<Self>()?.$($field).+.add_modifier |= ratatui::style::Modifier::BOLD;
			Ok(ud)
		});
		$methods.add_function("dim", |_, ud: mlua::AnyUserData| {
			ud.borrow_mut::<Self>()?.$($field).+.add_modifier |= ratatui::style::Modifier::DIM;
			Ok(ud)
		});
		$methods.add_function("italic", |_, ud: mlua::AnyUserData| {
			ud.borrow_mut::<Self>()?.$($field).+.add_modifier |= ratatui::style::Modifier::ITALIC;
			Ok(ud)
		});
		$methods.add_function("underline", |_, ud: mlua::AnyUserData| {
			ud.borrow_mut::<Self>()?.$($field).+.add_modifier |= ratatui::style::Modifier::UNDERLINED;
			Ok(ud)
		});
		$methods.add_function("blink", |_, ud: mlua::AnyUserData| {
			ud.borrow_mut::<Self>()?.$($field).+.add_modifier |= ratatui::style::Modifier::SLOW_BLINK;
			Ok(ud)
		});
		$methods.add_function("blink_rapid", |_, ud: mlua::AnyUserData| {
			ud.borrow_mut::<Self>()?.$($field).+.add_modifier |= ratatui::style::Modifier::RAPID_BLINK;
			Ok(ud)
		});
		$methods.add_function("reverse", |_, ud: mlua::AnyUserData| {
			ud.borrow_mut::<Self>()?.$($field).+.add_modifier |= ratatui::style::Modifier::REVERSED;
			Ok(ud)
		});
		$methods.add_function("hidden", |_, ud: mlua::AnyUserData| {
			ud.borrow_mut::<Self>()?.$($field).+.add_modifier |= ratatui::style::Modifier::HIDDEN;
			Ok(ud)
		});
		$methods.add_function("crossed", |_, ud: mlua::AnyUserData| {
			ud.borrow_mut::<Self>()?.$($field).+.add_modifier |= ratatui::style::Modifier::CROSSED_OUT;
			Ok(ud)
		});
		$methods.add_function("reset", |_, ud: mlua::AnyUserData| {
			ud.borrow_mut::<Self>()?.$($field).+.add_modifier = ratatui::style::Modifier::empty();
			Ok(ud)
		});
	};
}
