pub mod ticket;
pub mod ticket_comment;

pub use ticket::ActiveModel as TicketActiveModel;
pub use ticket::Column as TicketColumn;
pub use ticket::Entity as Ticket;

pub use ticket_comment::ActiveModel as TicketCommentActiveModel;
pub use ticket_comment::Column as TicketCommentColumn;
pub use ticket_comment::Entity as TicketComment;
