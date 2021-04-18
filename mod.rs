use block_tools::{
	blocks::{BlockType, Context, TypeInfo},
	display_api::{
		component::{atomic::icon::Icon, layout::card::CardComponent, DisplayComponent},
		CreationObject, DisplayObject,
	},
	models::{Block},
	LoopError,
};

pub const BLOCK_NAME: &str = "habit";

/// A block type that can be thought of like a sort of folder.
pub struct HabitBlock {
	pub name: Option<Block>,
	pub description: Option<Block>,
	pub items: Vec<Block>,
}

impl BlockType for HabitBlock {
	fn name() -> String {
		BLOCK_NAME.to_string()
	}

	fn info() -> TypeInfo {
		TypeInfo {
			name: Self::name(),
			icon: Icon::ThumbsUp,
			desc: "Group blocks organize blocks into folders.".to_string(),
		}
	}

	fn block_name(block: &Block, context: &Context) -> Result<String, LoopError> {
		todo!()
		// Self::handle_block_name(block, context)
	}

	fn page_display(block: &Block, context: &Context) -> Result<DisplayObject, LoopError> {
		todo!()
		// Self::handle_page_display(block, context)
	}

	fn embed_display(block: &Block, context: &Context) -> DisplayComponent {
		todo!()
		// Self::handle_embed_display(block, context)
		// 	.unwrap_or_else(|e| CardComponent::error_card(e).into())
	}

	fn create_display(context: &Context, user_id: i32) -> Result<CreationObject, LoopError> {
		todo!()
		// Self::handle_create_display(context, user_id)
	}

	fn create(input: String, context: &Context, user_id: i32) -> Result<Block, LoopError> {
		todo!()
		// Self::handle_create_raw(input, context, user_id)
	}

	fn method_delegate(
		context: &Context,
		name: String,
		block_id: i64,
		args: String,
	) -> Result<Block, LoopError> {
		todo!()
		// Self::handle_method_delegate(context, name, block_id, args)
	}

	fn visibility_update(context: &Context, block_id: i64, public: bool) -> Result<(), LoopError> {
		todo!()
		// Self::handle_visibility_update(context, block_id, public)
	}
}
