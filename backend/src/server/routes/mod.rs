pub mod auth_route;
pub mod boards_route;

pub use auth_route::route_init as auth_route_init;
pub use boards_route::route_init as board_route_init;
