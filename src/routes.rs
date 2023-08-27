use once_cell::sync::Lazy;

pub static ROUTING: Lazy<Route<'static, usize>> = Lazy::new(|| {
    Route::Nested(
        RouteData::new(0, "/", "Home"),
        vec![
            Route::Simple(RouteData::new(1, "/about", "About Us")),
            Route::Nested(
                RouteData::new(2, "/courses", "Tuition Services"),
                vec![
                    Route::Simple(RouteData::new(4, "/courses/french", "French")),
                    Route::Simple(RouteData::new(5, "/courses/german", "German")),
                ],
            ),
            Route::Simple(RouteData::new(3, "/contact", "Contact Us")),
        ],
    )
});

pub struct RouteData<'a, Idx> {
    pub id: Idx,
    pub route: &'a str,
    pub label: &'a str,
}

pub enum Route<'a, Idx> {
    Simple(RouteData<'a, Idx>),
    Nested(RouteData<'a, Idx>, Vec<Route<'a, Idx>>),
}

impl<'a, Idx> RouteData<'a, Idx> {
    pub fn new(id: Idx, route: &'a str, label: &'a str) -> Self {
        RouteData { id, route, label }
    }
}
