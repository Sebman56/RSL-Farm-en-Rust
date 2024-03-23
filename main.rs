
//#[macro_use] extern crate text_io;

//use text_io::scan;
use std::io;

fn main() {
    println!("\n\n\t Lancement du Calcul Raid Farm en Rust.\n");

    /* Déclaration des variables */

//    let mut SaisieEtoileChampion = 0;
//    let mut SaisieLevelChampion = 0;
    
    let mut XPChampion = 0 ;
    let mut XPCumuleChampion = 0;
    let mut XPTotalChampion = 0;
    let mut XPMaxChampion = 0;
    let mut XPManquantChampion = 0;
    
    let mut EnergieAraignee20 = 0;
    let mut EnergieDragon20 = 0;
    let mut EnergieCampagne12_6 = 0;
    
    let mut i = 0;
    
    let Dragon20 = 5782;
    let mut NRoundDragon20 = 0;
    
    let Araignee20 = 5800;
    let mut NRoundAraignee20 = 0;
    
    let CampagneBrutale12_6= 8800;
    let mut NRoundCampagneBrutale12_6 = 0;
    
    let mut NombreRound = 0;

/* **** Debut de tableau tableau afin connaitre le nombre XP par level **** */
let  ChampionEtoilesXP: [[usize; 60]; 6] = 
[
    [0,897,1118,1393,1736,2163,2695,3359,4185,5215,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,1110,1258,1426,1616,1831,2075,2351,2664,3019,3421,3876,4392,4977,5640,6390,7241,8205,9298,10536,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,1279,1411,1556,1717,1893,2088,2303,2541,2802,3091,3409,3760,4147,4574,5045,5565,6138,6769,7466,8235,9083,10018,11050,12188,13442,14827,16353,18037,19894,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0 ],
    [0,1460,1591,1734,1890,2060,2245,2447,2666,2906,3167,3451,3761,4099,4467,4868,5305,5782,6301,6867,7483,8155,8888,9686,10556,11504,12537,13663,14890,16227,17684,19272,21003,22889,24945,27185,29627,32287,35187,38347,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    [0,1625,1760,1907,2066,2238,2424,2626,2845,3082,3338,3616,3918,4244,4597,4980,5395,5844,6331,6858,7430,8048,8719,9445,10231,11084,12007,13007,14090,15264,16535,17912,19404,21020,22770,24667,26721,28947,31358,33970,36799,39864,43184,46780,50677,54897,59470,64423,69788,75601,0,0,0,0,0,0,0,0,0,0],
    [0,1813,1956,2110,2277,2457,2651,2860,3086,3329,3592,3876,4182,4512,4869,5253,5668,6115,6598,7119,7681,8288,8942,9648,10410,11232,12119,13076,14109,15223,16425,17722,19121,20631,22260,24018,25914,27961,30168,32551,35121,37894,40886,44115,47598,51357,55412,59787,64508,69602,75098,81028,87426,94329,101778,109815,118486,127842,137937,148828]
];

println!("\t Tableau pris en compte.\n");
/* **** Fin de tableau tableau **** */


// *** SaisieEtoileChampion)
println!("\n\n\t Entrez le nombre d'étoiles du champion:");

let mut SaisieEtoileChampion  = String::new();
io::stdin()
    .read_line(&mut SaisieEtoileChampion)
    .expect(" Echec de l'entrée utilisateur");
let SaisieEtoileChampion: usize = SaisieEtoileChampion.trim().parse().expect("Veuillez entrer un nombre !");

// println!("\t Vous avez rentré {} étoiles.",SaisieEtoileChampion);


// ** SaisieLevelChampion
println!("\n\t Entrez le level du champion:");
    let mut SaisieLevelChampion = String::new();
    io::stdin()
        .read_line(&mut SaisieLevelChampion)
        .expect(" Echec de l'entrée utilisateur");

        let SaisieLevelChampion: usize = SaisieLevelChampion.trim().parse().expect("Veuillez entrer un nombre !"); 

println!("\n\n\t Vous avez calculé pour un Champion --- {} --- étoiles pour un level numéro --- {} ---",SaisieEtoileChampion,SaisieLevelChampion);


/* *** Nombre d'XP du champion */
let  CEXP: usize  =  ChampionEtoilesXP [SaisieEtoileChampion-1][SaisieLevelChampion];
println!("\nNombre d'XP du champion: {}.", CEXP );

/* *** Nombre d'XP maximum du champion en fonction de son nombre d'étoiles */
let XPMaxChampion: usize  ; 
match SaisieEtoileChampion {
    1 => {XPMaxChampion = 22761;}
    2 => {XPMaxChampion = 81326;}
    3 => { XPMaxChampion = 200681;}
    4 => { XPMaxChampion = 449082;}
    5 => { XPMaxChampion = 963806;}
    6 => { XPMaxChampion = 2010869;}
    _ => { XPMaxChampion = 0; println!(" Erreur de saisie");}
};         
println!("XP Max d'un champion {} étoiles: {}.", SaisieEtoileChampion, XPMaxChampion);


/* *** Nombre d’XP manquant */
let XPmanquant: usize ;
XPmanquant = XPMaxChampion - CEXP;
println! ("*** {} *** XP manquant du Champion.\n", XPmanquant);

/* ***Round néc éssaires */
let NRoundDragon20: usize ;
NRoundDragon20 = XPmanquant / Dragon20 ;
println! ("\t*** {} *** Rounds nécéssaires pour qu’un champion {} étoiles atteigne son maximum dans le Dragon 20.\n", NRoundDragon20, SaisieEtoileChampion);


let NRoundAraignee20 ;
NRoundAraignee20 = XPmanquant /Araignee20 ;
println! ("\t*** {} *** Rounds nécéssaires pour qu’un champion {} étoiles atteigne son maximum dans l’araignée 20.\n", NRoundAraignee20, SaisieEtoileChampion);

let NRoundCampagneBrutale12_6 ;
NRoundCampagneBrutale12_6 = XPmanquant / CampagneBrutale12_6 ;
println! ("\t*** {} *** Rounds nécéssaires pour qu’un champion {} étoiles atteigne son maximum en campagne brutale 12-6.\n", NRoundCampagneBrutale12_6, SaisieEtoileChampion);
}
