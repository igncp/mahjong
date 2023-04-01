use crate::{Dragon, Flower, Season, Suit, Tile, Wind};

pub fn format_to_emoji(tile: &Tile) -> String {
    match tile {
        Tile::Suit(tile) => match tile.suit {
            Suit::Bamboo => format!("🎋{}", tile.value),
            Suit::Characters => format!("✨{}", tile.value),
            Suit::Dots => format!("💠{}", tile.value),
        },
        Tile::Wind(tile) => match tile.value {
            Wind::East => "🍃EA".to_string(),
            Wind::North => "🍃NO".to_string(),
            Wind::South => "🍃SO".to_string(),
            Wind::West => "🍃WE".to_string(),
        },
        Tile::Dragon(tile) => match tile.value {
            Dragon::Green => "🐉GR".to_string(),
            Dragon::Red => "🐉RE".to_string(),
            Dragon::White => "🐉WH".to_string(),
        },
        Tile::Flower(tile) => match tile.value {
            Flower::Bamboo => "💮BA".to_string(),
            Flower::Chrysanthemum => "💮CH".to_string(),
            Flower::Orchid => "💮OR".to_string(),
            Flower::Plum => "💮PL".to_string(),
        },
        Tile::Season(tile) => match tile.value {
            Season::Autumn => "🌞AU".to_string(),
            Season::Spring => "🌞SP".to_string(),
            Season::Summer => "🌞SU".to_string(),
            Season::Winter => "🌞WI".to_string(),
        },
    }
}
