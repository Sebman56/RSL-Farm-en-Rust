use iced::widget::{button, column, combo_box, container, row, text};
use iced::{Alignment, Element, Length, Task, application, window, Theme};

// ==================== MESSAGE ====================
#[derive(Debug, Clone)]
enum Message {
    EtoilesSelected(usize),
    LevelSelected(usize),
    Calculer,
    FermerResultats,
}

// ==================== STRUCTURE ====================
#[derive(Default)]
struct RaidFarm {
    etoiles: usize,
    level: usize,
    resultat: Option<Resultats>,

    etoiles_state: combo_box::State<usize>,
    level_state: combo_box::State<usize>,
}

#[derive(Debug, Clone)]
struct Resultats {
    etoiles: usize,
    level: usize,
    xp_actuel: usize,
    xp_max: usize,
    xp_manquant: usize,
    rounds_dragon20: usize,
    energie_dragon20: usize,
    rounds_araignee20: usize,
    energie_araignee20: usize,
    rounds_campagne: usize,
    energie_campagne: usize,
}

// ==================== INITIALISATION ====================
fn new() -> RaidFarm {
    RaidFarm {
        etoiles: 1,
        level: 1,
        resultat: None,
        etoiles_state: combo_box::State::new((1..=6).collect()),
        level_state: combo_box::State::new((1..=10).collect()),
    }
}

fn update(app: &mut RaidFarm, message: Message) -> Task<Message> {
    match message {
        Message::EtoilesSelected(e) => {
            app.etoiles = e;
            app.level = 1;

            let max_level = match e {
                1 => 10, 2 => 20, 3 => 30, 4 => 40, 5 => 50, 6 => 60,
                _ => 60,
            };
            app.level_state = combo_box::State::new((1..=max_level).collect());
            app.resultat = None;
        }
        Message::LevelSelected(l) => {
            app.level = l;
            app.resultat = None;
        }
        Message::Calculer => {
            if (1..=6).contains(&app.etoiles) && app.level >= 1 {
                app.resultat = Some(calculer_resultats(app.etoiles, app.level));
            }
        }
        Message::FermerResultats => app.resultat = None,
    }
    Task::none()
}

fn view(app: &RaidFarm) -> Element<Message> {
    let controls = column![
        row![
            text("Nombre d'étoiles :").width(Length::FillPortion(1)),
            combo_box(&app.etoiles_state, "Choisissez le nombre d'étoiles", Some(&app.etoiles), Message::EtoilesSelected)
                .width(Length::FillPortion(1)),
        ].spacing(10).align_y(Alignment::Center),

        row![
            text("Niveau du champion :").width(Length::FillPortion(1)),
            combo_box(&app.level_state, "Choisissez le niveau", Some(&app.level), Message::LevelSelected)
                .width(Length::FillPortion(1)),
        ].spacing(10).align_y(Alignment::Center),

        button("Calculer").on_press(Message::Calculer).padding(12),
    ]
    .spacing(25)
    .padding(30)
    .align_x(Alignment::Center);

    let content = if let Some(res) = &app.resultat {
        column![
            text("Résultats du calcul").size(28),
            text(format!("Champion {} ★ — Niveau {}", res.etoiles, res.level)).size(20),
            text(""),
            text(format!("XP actuel      : {}", format_nombre(res.xp_actuel))),
            text(format!("XP maximum     : {}", format_nombre(res.xp_max))),
            text(format!("XP manquant    : {}", format_nombre(res.xp_manquant))),
            text(""),
            text(format!("🐉 Dragon 20     : {} rounds → {} énergie", res.rounds_dragon20, format_nombre(res.energie_dragon20))),
            text(format!("🕷️ Araignée 20   : {} rounds → {} énergie", res.rounds_araignee20, format_nombre(res.energie_araignee20))),
            text(format!("🏹 Campagne 12-6 : {} rounds → {} énergie", res.rounds_campagne, format_nombre(res.energie_campagne))),
            button("Fermer les résultats").on_press(Message::FermerResultats).padding(10),
        ]
        .spacing(18).padding(40).align_x(Alignment::Center)
    } else {
        controls
    };

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

// Formatage des grands nombres avec espace
fn format_nombre(n: usize) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let mut count = 0;
    for c in s.chars().rev() {
        if count == 3 {
            result.push(' ');
            count = 0;
        }
        result.push(c);
        count += 1;
    }
    result.chars().rev().collect()
}

// ==================== TABLEAU XP (corrigé) ====================
const CHAMPION_XP_TABLE: [[usize; 61]; 7] = [
    [0; 61],
    // 1 étoile
    [0,897,1118,1393,1736,2163,2695,3359,4185,5215,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    // 2 étoiles
    [0,1110,1258,1426,1616,1831,2075,2351,2664,3019,3421,3876,4392,4977,5640,6390,7241,8205,9298,10536,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    // 3 étoiles
    [0,1279,1411,1556,1717,1893,2088,2303,2541,2802,3091,3409,3760,4147,4574,5045,5565,6138,6769,7466,8235,9083,10018,11050,12188,13442,14827,16353,18037,19894,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    // 4 étoiles
    [0,1460,1591,1734,1890,2060,2245,2447,2666,2906,3167,3451,3761,4099,4467,4868,5305,5782,6301,6867,7483,8155,8888,9686,10556,11504,12537,13663,14890,16227,17684,19272,21003,22889,24945,27185,29627,32287,35187,38347,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    // 5 étoiles
    [0,1625,1760,1907,2066,2238,2424,2626,2845,3082,3338,3616,3918,4244,4597,4980,5395,5844,6331,6858,7430,8048,8719,9445,10231,11084,12007,13007,14090,15264,16535,17912,19404,21020,22770,24667,26721,28947,31358,33970,36799,39864,43184,46780,50677,54897,59470,64423,69788,75601,0,0,0,0,0,0,0,0,0,0,0],
    // 6 étoiles (corrigé : 61 éléments)
    [0,1813,1956,2110,2277,2457,2651,2860,3086,3329,3592,3876,4182,4512,4869,5253,5668,6115,6598,7119,7681,8288,8942,9648,10410,11232,12119,13076,14109,15223,16425,17722,19121,20631,22260,24018,25914,27961,30168,32551,35121,37894,40886,44115,47598,51357,55412,59787,64508,69602,75098,81028,87426,94329,101778,109815,118486,127842,137937,148828,0],
];

const DRAGON20: usize = 5782;
const ARAIGNEE20: usize = 5800;
const CAMPAGNE12_6: usize = 8800;

fn calculer_resultats(etoiles: usize, level: usize) -> Resultats {
    let xp_actuel = if level < 61 { CHAMPION_XP_TABLE[etoiles][level] } else { 0 };

    let xp_max = match etoiles {
        1 => 22761,
        2 => 81326,
        3 => 200681,
        4 => 449082,
        5 => 963806,
        6 => 2010869,
        _ => 0,
    };

    let xp_manquant = if xp_max > xp_actuel { xp_max - xp_actuel } else { 0 };

    Resultats {
        etoiles,
        level,
        xp_actuel,
        xp_max,
        xp_manquant,
        rounds_dragon20: xp_manquant / DRAGON20,
        energie_dragon20: (xp_manquant / DRAGON20) * 16,
        rounds_araignee20: xp_manquant / ARAIGNEE20,
        energie_araignee20: (xp_manquant / ARAIGNEE20) * 16,
        rounds_campagne: xp_manquant / CAMPAGNE12_6,
        energie_campagne: (xp_manquant / CAMPAGNE12_6) * 8,
    }
}

fn theme(_: &RaidFarm) -> Theme {
    Theme::Dark
}

// ==================== MAIN ====================
fn main() -> iced::Result {
    application(new, update, view)
        .theme(theme)
        .window(window::Settings {
            size: iced::Size::new(720.0, 600.0),
            ..Default::default()
        })
        .run()
}