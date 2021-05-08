use crate::blocks::habit_block::HabitBlock;
use block_tools::{
	auth::{
		optional_token, optional_validate_token,
		permissions::{has_perm_level, PermLevel},
	},
	blocks::Context,
	display_api::component::{
		atomic::text::TextComponent,
		interact::link::LinkComponent,
		layout::{
			card::{CardComponent, DetachedMenu},
			stack::{AlignXOptions, StackComponent},
		},
		menus::menu::MenuComponent,
		DisplayComponent,
	},
	models::Block,
	LoopError,
};

impl HabitBlock {
	pub fn handle_embed_display(
		block: &Block,
		context: &Context,
	) -> Result<DisplayComponent, LoopError> {
		let conn = &context.conn()?;
		let user_id = optional_validate_token(optional_token(context))?;

		let Self {
			name,
			description,
			impact,
			score,
			streak,
		} = Self::from_id(block.id, user_id, conn)?;

		let mut left_col = StackComponent::vertical();
		let mut middle_col = StackComponent::vertical();
		let mut right_col = StackComponent {
			align_x: Some(AlignXOptions::Right),
			..StackComponent::vertical()
		};

		let name = name
			.and_then(|block| block.block_data)
			.unwrap_or_else(|| "Untitled Habit".into());
		let text = TextComponent {
			bold: Some(true),
			..TextComponent::new(name)
		};
		let link = LinkComponent {
			app_path: Some(format!("/b/{}", block.id)),
			no_style: Some(true),
			..LinkComponent::new(text)
		};
		middle_col.push(link);

		if let Some(desc) = Self::description(&description, false) {
			middle_col.push(desc)
		}

		let mut detached_menu = None;

		if let Some(user_id) = user_id {
			if has_perm_level(user_id, block, PermLevel::Edit) {
				let buttons_stack = Self::buttons_stack(impact, block.id);
				right_col.push(buttons_stack);
				let mut menu = MenuComponent::from_block(block, user_id);
				menu.load_comments(conn)?;
				detached_menu = Some(DetachedMenu::bottom_right(menu));
			}
		}

		right_col.push(Self::streak(streak));

		left_col.push(Self::score_circle(score, block));

		let mut content = StackComponent::horizontal();
		content.push(left_col);
		content.push(middle_col);
		content.push(right_col);

		Ok(CardComponent {
			color: block.color.clone(),
			detached_menu,
			..CardComponent::new(content)
		}
		.into())
	}
}
