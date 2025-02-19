//! Data visualize
use crate::attr::{Attr, Count, Owner};
use crate::res::{Res, Set, Sets};
use crate::status::{Status, StatusFlags};
use tabled::{
    builder::Builder,
    settings::{object::Rows, Alignment, Style},
};

/// Visualize the sets as table
pub trait TableSets: Sets {
    /// Visualize the sets as table
    fn table(&self);
}
/// Visualize the set as table
pub trait TableSet: Set {
    /// Visualize the set as table
    fn table(&self);
}
/// Visualize the resource as table
pub trait TableRes: Res {
    /// Visualize the resource as table
    fn table(&self);
}

impl<T> TableSets for T
where
    T: Sets,
    T::Set: Attr + Count + Status,
{
    fn table(&self) {
        let header = vec!["ID", "Title", "Count", "Track"];
        let rows = self.iter().map(|set| {
            let id = String::from(set.id());
            let title = set.title().to_string();
            let count = set.count().to_string();
            let track = set.check_status(StatusFlags::TRACK).to_string();
            vec![id, title, count, track]
        });
        show_table(header, rows);
    }
}

impl<T> TableSet for T
where
    T: Set,
    T::Res: Attr + Status + Owner,
{
    fn table(&self) {
        let header = vec!["ID", "Upper", "Title", "Track", "Saved"];
        let rows = self.iter().map(|res| {
            let id = String::from(res.id());
            let upper = res.owner().to_string().chars().take(15).collect();
            let title = res.title().to_string().chars().take(15).collect();
            let track = res.check_status(StatusFlags::TRACK).to_string();
            let saved = res.check_status(StatusFlags::SAVED).to_string();
            vec![id, upper, title, track, saved]
        });
        show_table(header, rows);
    }
}

impl<T> TableRes for T
where
    T: Res,
{
    fn table(&self) {
        let header = vec!["ID", "Upper", "Title", "Track", "Saved", "Expired"];
        let id = String::from(self.id());
        let upper = self.owner().to_string().chars().take(15).collect();
        let title = self.title().to_string().chars().take(15).collect();
        let track = self.check_status(StatusFlags::TRACK).to_string();
        let saved = self.check_status(StatusFlags::SAVED).to_string();
        let expired = self.check_status(StatusFlags::EXPIRED).to_string();
        let rows = vec![vec![id, upper, title, track, saved, expired]];
        show_table(header, rows);
    }
}

/// Show table helper function
/// # Example
/// ```
/// # #[cfg(feature = "visual")]
/// # {
/// use fav_core::visual::show_table;
/// let header = vec!["ID", "Title", "Track", "Saved"];
/// let rows = vec![vec!["1", "Title", "true", "false"], vec!["2", "Title", "false", "true"]];
/// show_table(header, rows);
/// # }
/// ```
pub fn show_table<H, R>(header: H, rows: R)
where
    H: IntoIterator,
    H::Item: Into<String>,
    R: IntoIterator,
    R::Item: IntoIterator,
    <R::Item as IntoIterator>::Item: Into<String>,
{
    let mut builder = Builder::default();
    builder.push_record(header);
    let mut count = 0;
    rows.into_iter().for_each(|r| {
        count += 1;
        builder.push_record(r);
    });
    match count {
        0 => {}
        _ => {
            let table = builder
                .build()
                .with(Style::modern())
                .modify(Rows::new(1..), Alignment::left())
                .to_string();
            println!("{}", table);
            println!("Count: {}\n", count);
        }
    }
}
