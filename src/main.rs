

pub type EntityId = String;
pub type RegionId = String;
pub type ContinentId = String;
pub type Utroopcount = u16;

#[derive(Debug)]
pub enum Color {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    White,
    Gray,
    Yellow
}

#[derive(Debug, PartialEq)]
pub enum Occupant {
    HumanPlayer(EntityId),
}

#[derive(Debug)]
pub struct Region {
    id: RegionId,
    occupant: Occupant,
    troop_count: Utroopcount,
}

impl Region {
    pub fn new(region_id: RegionId, occupant: Occupant) -> Self {
        Region {
            id: region_id,
            occupant,
            troop_count: 1
        }
    }
}

pub type Continent = Vec<Region>;
pub type World = Vec<Continent>;

#[derive(Debug)]
pub struct Player {
    id: EntityId,
    name: String,
    color: Color,
}

impl Player {
    pub fn new(player_id: EntityId) -> Self {
        Player {
            id: player_id,
            name: "".to_string(),
            color: Color::Blue
        }
    }
}

pub fn compute_reinforcements(player_id: String, world: &World) -> Utroopcount {
    let region_control_component = world.iter()
        .flat_map(|c| c.iter().filter(|r| r.occupant == Occupant::HumanPlayer(player_id.clone())))
        .fold(0, |a, b| a + 1);
    // println!("{:?}", player_controlled_regions);
    return std::cmp::max(3, region_control_component / 3);
}

#[cfg(test)]
pub mod tests {
    use crate::*;

    #[test]
    pub fn simple_test_001() {
        let player_1 = Player::new("1".to_string());
        let player_2 = Player::new("2".to_string());
        let mut egypt =
            Region::new(
                "Egypt".to_string(),
                Occupant::HumanPlayer(player_1.id.clone()));
        let mut north_africa =
            Region::new(
                "North Africa".to_string(),
                Occupant::HumanPlayer(player_2.id.clone()));


        let mut world: World = Vec::new();
        let mut africa: Continent = Vec::new();
        africa.push(egypt);
        africa.push(north_africa);
        world.push(africa);

        let expected: Utroopcount = 3;
        let actual = compute_reinforcements(player_1.id, &world);
        assert_eq!(actual, expected);

    }
}


fn main() {
    println!("Hello, world!");
}
