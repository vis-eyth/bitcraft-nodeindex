use bindings::region::EnemyType;

pub trait Enum where Self: Sized {
    fn values() -> &'static [Self];
}

const ENEMY_TYPE: [EnemyType; 32] = [
    EnemyType::None,
    EnemyType::PracticeDummy,
    EnemyType::GrassBird,
    EnemyType::DesertBird,
    EnemyType::SwampBird,
    EnemyType::Goat,
    EnemyType::MountainGoat,
    EnemyType::DeerFemale,
    EnemyType::DeerMale,
    EnemyType::Elk,
    EnemyType::BoarFemale,
    EnemyType::BoarMale,
    EnemyType::BoarElder,
    EnemyType::PlainsOx,
    EnemyType::TundraOx,
    EnemyType::JungleLargeBird,
    EnemyType::DesertLargeBird,
    EnemyType::Jakyl,
    EnemyType::AlphaJakyl,
    EnemyType::KingJakyl,
    EnemyType::RockCrab,
    EnemyType::DesertCrab,
    EnemyType::FrostCrab,
    EnemyType::ForestToad,
    EnemyType::SwampToad,
    EnemyType::FrostToad,
    EnemyType::Umbura,
    EnemyType::AlphaUmbura,
    EnemyType::KingUmbura,
    EnemyType::Drone,
    EnemyType::Soldier,
    EnemyType::Queen,
];

impl Enum for EnemyType {
    fn values() -> &'static [Self] { &ENEMY_TYPE }
}