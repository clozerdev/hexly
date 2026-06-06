use adw::prelude::*;

use crate::core::state::{BlockDump, BlockKind, CardData, KeyType, SectorDump};

pub(crate) fn populate_sectors(container: &gtk::Box, data: &CardData) -> Vec<adw::ExpanderRow> {
    clear_box_children(container);

    let mut expanders = Vec::with_capacity(data.sectors.len());

    for sector in &data.sectors {
        let (group, expander) = build_sector_group(sector);
        container.append(&group);
        expanders.push(expander);
    }

    expanders
}

pub(crate) fn all_sectors_expanded(expanders: &[adw::ExpanderRow]) -> bool {
    !expanders.is_empty() && expanders.iter().all(adw::ExpanderRow::is_expanded)
}

pub(crate) fn set_all_sectors_expanded(expanders: &[adw::ExpanderRow], expanded: bool) {
    for expander in expanders {
        expander.set_expanded(expanded);
    }
}

pub(crate) fn clear_box_children(container: &gtk::Box) {
    while let Some(child) = container.first_child() {
        container.remove(&child);
    }
}

pub(crate) fn build_badge_label(text: &str, css_class: &str) -> gtk::Label {
    let badge = gtk::Label::new(Some(text));

    badge.set_valign(gtk::Align::Center);
    badge.set_vexpand(false);
    badge.set_margin_top(2);
    badge.set_margin_bottom(2);
    badge.add_css_class("hexly-badge");
    badge.add_css_class(css_class);
    badge
}

pub(crate) fn block_kind_label(kind: &BlockKind) -> &'static str {
    match kind {
        BlockKind::Manufacturer => "Manufacturer",
        BlockKind::Data => "Data",
        BlockKind::Trailer => "Trailer",
    }
}

// Private functions
fn build_sector_group(sector: &SectorDump) -> (adw::PreferencesGroup, adw::ExpanderRow) {
    let group = adw::PreferencesGroup::new();
    let expander = build_sector_expander(sector);

    group.set_hexpand(true);
    group.add(&expander);

    (group, expander)
}

fn build_sector_expander(sector: &SectorDump) -> adw::ExpanderRow {
    let row = adw::ExpanderRow::new();
    row.set_title(&format!("Sector {}", sector.sector_index));
    row.set_expanded(sector.sector_index == 0);
    row.add_suffix(&build_sector_header_suffix(sector));

    for block in &sector.blocks {
        row.add_row(&build_block_row(block));
    }

    row
}

fn build_sector_header_suffix(sector: &SectorDump) -> gtk::Box {
    let badges = gtk::Box::new(gtk::Orientation::Horizontal, 6);

    let auth_badge = match sector.authenticated_with {
        Some(KeyType::A) => build_badge_label("Auth: Key A", "hexly-badge-auth-success"),
        Some(KeyType::B) => build_badge_label("Auth: Key B", "hexly-badge-auth-success"),
        None => build_badge_label("Auth failed", "hexly-badge-auth-failed"),
    };
    badges.append(&auth_badge);

    badges
}

fn build_block_row(block: &BlockDump) -> adw::ActionRow {
    let title = format!(
        "Block {} · {}",
        block.block_index,
        block_kind_label(&block.kind)
    );
    let subtitle = match block.data {
        Some(bytes) => bytes
            .iter()
            .map(|b| format!("{b:02X}"))
            .collect::<Vec<_>>()
            .join(" "),
        None => "Unreadable".to_string(),
    };

    let row = adw::ActionRow::new();
    row.add_css_class("property");
    row.add_css_class("hexly-block-row");
    row.set_title(&title);
    row.set_subtitle(&subtitle);

    match block.kind {
        BlockKind::Manufacturer => {
            let badge = build_badge_label("Manufacturer", "hexly-badge-manufacturer");

            row.add_css_class("hexly-row-manufacturer");
            row.add_suffix(&badge);
        }
        BlockKind::Trailer => {
            let badge = build_badge_label("Trailer · Keys", "hexly-badge-trailer");

            row.add_css_class("hexly-row-trailer");
            row.add_suffix(&badge);
        }
        BlockKind::Data => {}
    }

    row
}
