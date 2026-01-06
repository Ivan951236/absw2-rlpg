use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget},
    Frame, Terminal,
};
use std::io::{self};

#[derive(Debug, Clone)]
enum Side {
    Bird,
    Pork,
}

#[derive(Debug, Clone)]
struct Character {
    name: String,
    side: Side,
}

#[derive(Debug, Clone)]
enum AppState {
    SelectingSide,
    GeneratingPreset,
    ShowingPreset,
}

#[derive(Debug, Clone)]
struct LevelPreset {
    world: u32,
    level: u32,
    characters: Vec<Character>,
}

#[derive(Debug, Clone)]
struct App {
    state: AppState,
    selected_side: Option<Side>,
    level_presets: Vec<LevelPreset>,
    current_input: String,
    grid_shape: GridShape,
    current_language: Language,
}

#[derive(Debug, Clone)]
enum GridShape {
    Square,
    Stages,
}

#[derive(Debug, Clone)]
enum Language {
    English,
    Spanish,
    French,
    German,
    Italian,
    Portuguese,
    Dutch,
    Russian,
    Chinese,
    Japanese,
    Korean,
    Arabic,
    Hindi,
    Turkish,
    Polish,
    Czech,
    Swedish,
    Finnish,
    Norwegian,
    Danish,
}

impl Language {
    fn next_language(&self) -> Self {
        match self {
            Language::English => Language::Spanish,
            Language::Spanish => Language::French,
            Language::French => Language::German,
            Language::German => Language::Italian,
            Language::Italian => Language::Portuguese,
            Language::Portuguese => Language::Dutch,
            Language::Dutch => Language::Russian,
            Language::Russian => Language::Chinese,
            Language::Chinese => Language::Japanese,
            Language::Japanese => Language::Korean,
            Language::Korean => Language::Arabic,
            Language::Arabic => Language::Hindi,
            Language::Hindi => Language::Turkish,
            Language::Turkish => Language::Polish,
            Language::Polish => Language::Czech,
            Language::Czech => Language::Swedish,
            Language::Swedish => Language::Finnish,
            Language::Finnish => Language::Norwegian,
            Language::Norwegian => Language::Danish,
            Language::Danish => Language::English,
        }
    }

    fn get_language_code(&self) -> &str {
        match self {
            Language::English => "EN",
            Language::Spanish => "ES",
            Language::French => "FR",
            Language::German => "DE",
            Language::Italian => "IT",
            Language::Portuguese => "PT",
            Language::Dutch => "NL",
            Language::Russian => "RU",
            Language::Chinese => "ZH",
            Language::Japanese => "JA",
            Language::Korean => "KO",
            Language::Arabic => "AR",
            Language::Hindi => "HI",
            Language::Turkish => "TR",
            Language::Polish => "PL",
            Language::Czech => "CS",
            Language::Swedish => "SV",
            Language::Finnish => "FI",
            Language::Norwegian => "NO",
            Language::Danish => "DA",
        }
    }
}

impl App {
    fn new() -> App {
        App {
            state: AppState::SelectingSide,
            selected_side: None,
            level_presets: Vec::new(),
            current_input: String::new(),
            grid_shape: GridShape::Square,
            current_language: Language::English,
        }
    }

    fn get_translation(&self, key: &str, language: &Language) -> String {
        match language {
            Language::English => self.get_english_text(key),
            Language::Spanish => self.get_spanish_text(key),
            Language::French => self.get_french_text(key),
            Language::German => self.get_german_text(key),
            Language::Italian => self.get_italian_text(key),
            Language::Portuguese => self.get_portuguese_text(key),
            Language::Dutch => self.get_dutch_text(key),
            Language::Russian => self.get_russian_text(key),
            Language::Chinese => self.get_chinese_text(key),
            Language::Japanese => self.get_japanese_text(key),
            Language::Korean => self.get_korean_text(key),
            Language::Arabic => self.get_arabic_text(key),
            Language::Hindi => self.get_hindi_text(key),
            Language::Turkish => self.get_turkish_text(key),
            Language::Polish => self.get_polish_text(key),
            Language::Czech => self.get_czech_text(key),
            Language::Swedish => self.get_swedish_text(key),
            Language::Finnish => self.get_finnish_text(key),
            Language::Norwegian => self.get_norwegian_text(key),
            Language::Danish => self.get_danish_text(key),
        }
    }

    fn get_english_text(&self, key: &str) -> String {
        match key {
            "title" => "Angry Birds Star Wars 2 Roguelike Preset Generator".to_string(),
            "bird_side" => "1. Bird Side".to_string(),
            "pork_side" => "2. Pork Side".to_string(),
            "choose_side" => "Choose Side".to_string(),
            "instructions_selecting" => "Press 1 or 2 to select a side. Press 'H' to toggle grid shape.".to_string(),
            "instructions_showing" => "Press Enter or Esc to generate new presets. Press 'H' to toggle grid shape.".to_string(),
            "bird" => "Bird".to_string(),
            "pork" => "Pork".to_string(),
            "level_format" => format!("Level {} (World {})", 1, 1), // Placeholder
            "grid_shape_current" => "Square".to_string(),
            "grid_shape_hexagon" => "Stages".to_string(),
            "grid_shape_toggle" => "Press 'J' to change language".to_string(),
            "language_indicator" => format!("Language: {}", "EN"), // Placeholder
            _ => key.to_string(),
        }
    }

    fn get_spanish_text(&self, key: &str) -> String {
        match key {
            "title" => "Generador de Presets para Angry Birds Star Wars 2 en Modo Roguelike".to_string(),
            "bird_side" => "1. Lado de los Pájaros".to_string(),
            "pork_side" => "2. Lado de los Cerditos".to_string(),
            "choose_side" => "Elegir Lado".to_string(),
            "instructions_selecting" => "Presiona 1 o 2 para seleccionar un lado. Presiona 'H' para cambiar la forma de la cuadrícula.".to_string(),
            "instructions_showing" => "Presiona Enter o Esc para generar nuevos presets. Presiona 'H' para cambiar la forma de la cuadrícula.".to_string(),
            "bird" => "Pájaro".to_string(),
            "pork" => "Cerdito".to_string(),
            "level_format" => format!("Nivel {} (Mundo {})", 1, 1), // Placeholder
            "grid_shape_current" => "Cuadrado".to_string(),
            "grid_shape_hexagon" => "Etapas".to_string(),
            "grid_shape_toggle" => "Presiona 'J' para cambiar idioma".to_string(),
            "language_indicator" => format!("Idioma: {}", "ES"), // Placeholder
            _ => key.to_string(),
        }
    }

    fn get_french_text(&self, key: &str) -> String {
        match key {
            "title" => "Générateur de Préréglages Angry Birds Star Wars 2 en Mode Roguelike".to_string(),
            "bird_side" => "1. Côté Oiseaux".to_string(),
            "pork_side" => "2. Côté Cochons".to_string(),
            "choose_side" => "Choisir le Côté".to_string(),
            "instructions_selecting" => "Appuyez sur 1 ou 2 pour sélectionner un côté. Appuyez sur 'H' pour basculer la forme de la grille.".to_string(),
            "instructions_showing" => "Appuyez sur Entrée ou Échap pour générer de nouveaux préréglages. Appuyez sur 'H' pour basculer la forme de la grille.".to_string(),
            "bird" => "Oiseau".to_string(),
            "pork" => "Cochon".to_string(),
            "level_format" => format!("Niveau {} (Monde {})", 1, 1), // Placeholder
            "grid_shape_current" => "Carré".to_string(),
            "grid_shape_hexagon" => "Étapes".to_string(),
            "grid_shape_toggle" => "Appuyez sur 'J' pour changer de langue".to_string(),
            "language_indicator" => format!("Langue: {}", "FR"), // Placeholder
            _ => key.to_string(),
        }
    }

    fn get_german_text(&self, key: &str) -> String {
        match key {
            "title" => "Angry Birds Star Wars 2 Roguelike Preset Generator".to_string(),
            "bird_side" => "1. Vogelseite".to_string(),
            "pork_side" => "2. Schweineseite".to_string(),
            "choose_side" => "Seite wählen".to_string(),
            "instructions_selecting" => "Drücken Sie 1 oder 2, um eine Seite auszuwählen. Drücken Sie 'H', um die Gitterform zu wechseln.".to_string(),
            "instructions_showing" => "Drücken Sie Enter oder Esc, um neue Presets zu generieren. Drücken Sie 'H', um die Gitterform zu wechseln.".to_string(),
            "bird" => "Vogel".to_string(),
            "pork" => "Schwein".to_string(),
            "level_format" => format!("Level {} (Welt {})", 1, 1), // Placeholder
            "grid_shape_current" => "Quadrat".to_string(),
            "grid_shape_hexagon" => "Stufen".to_string(),
            "grid_shape_toggle" => "Drücken Sie 'J', um die Sprache zu ändern".to_string(),
            "language_indicator" => format!("Sprache: {}", "DE"), // Placeholder
            _ => key.to_string(),
        }
    }

    fn get_italian_text(&self, key: &str) -> String {
        match key {
            "title" => "Generatore di Preset per Angry Birds Star Wars 2 in Modalità Roguelike".to_string(),
            "bird_side" => "1. Lato Uccelli".to_string(),
            "pork_side" => "2. Lato Maiali".to_string(),
            "choose_side" => "Scegli Lato".to_string(),
            "instructions_selecting" => "Premi 1 o 2 per selezionare un lato. Premi 'H' per cambiare la forma della griglia.".to_string(),
            "instructions_showing" => "Premi Invio o Esc per generare nuovi preset. Premi 'H' per cambiare la forma della griglia.".to_string(),
            "bird" => "Uccello".to_string(),
            "pork" => "Maiale".to_string(),
            "level_format" => format!("Livello {} (Mondo {})", 1, 1), // Placeholder
            "grid_shape_current" => "Quadrato".to_string(),
            "grid_shape_hexagon" => "Fasi".to_string(),
            "grid_shape_toggle" => "Premi 'J' per cambiare lingua".to_string(),
            "language_indicator" => format!("Lingua: {}", "IT"), // Placeholder
            _ => key.to_string(),
        }
    }

    fn get_portuguese_text(&self, key: &str) -> String {
        match key {
            "title" => "Gerador de Predefinições do Angry Birds Star Wars 2 no Modo Roguelike".to_string(),
            "bird_side" => "1. Lado dos Pássaros".to_string(),
            "pork_side" => "2. Lado dos Porcos".to_string(),
            "choose_side" => "Escolher Lado".to_string(),
            "instructions_selecting" => "Pressione 1 ou 2 para selecionar um lado. Pressione 'H' para alternar a forma da grade.".to_string(),
            "instructions_showing" => "Pressione Enter ou Esc para gerar novas predefinições. Pressione 'H' para alternar a forma da grade.".to_string(),
            "bird" => "Pássaro".to_string(),
            "pork" => "Porco".to_string(),
            "level_format" => format!("Nível {} (Mundo {})", 1, 1), // Placeholder
            "grid_shape_current" => "Quadrado".to_string(),
            "grid_shape_hexagon" => "Etapas".to_string(),
            "grid_shape_toggle" => "Pressione 'J' para mudar idioma".to_string(),
            "language_indicator" => format!("Idioma: {}", "PT"), // Placeholder
            _ => key.to_string(),
        }
    }

    fn get_dutch_text(&self, key: &str) -> String {
        match key {
            "title" => "Angry Birds Star Wars 2 Roguelike Preset Generator".to_string(),
            "bird_side" => "1. Vogelzijde".to_string(),
            "pork_side" => "2. Varkenszijde".to_string(),
            "choose_side" => "Kies Zijde".to_string(),
            "instructions_selecting" => "Druk op 1 of 2 om een zijde te kiezen. Druk op 'H' om de raster vorm te wisselen.".to_string(),
            "instructions_showing" => "Druk op Enter of Esc om nieuwe presets te genereren. Druk op 'H' om de raster vorm te wisselen.".to_string(),
            "bird" => "Vogel".to_string(),
            "pork" => "Varken".to_string(),
            "level_format" => format!("Level {} (Wereld {})", 1, 1), // Placeholder
            "grid_shape_current" => "Vierkant".to_string(),
            "grid_shape_hexagon" => "Stadia".to_string(),
            "grid_shape_toggle" => "Druk op 'J' om van taal te wisselen".to_string(),
            "language_indicator" => format!("Taal: {}", "NL"), // Placeholder
            _ => key.to_string(),
        }
    }

    fn get_russian_text(&self, key: &str) -> String {
        match key {
            "title" => "Генератор пресетов Angry Birds Star Wars 2 в стиле рогалик".to_string(),
            "bird_side" => "1. Сторона птиц".to_string(),
            "pork_side" => "2. Сторона свиней".to_string(),
            "choose_side" => "Выберите сторону".to_string(),
            "instructions_selecting" => "Нажмите 1 или 2, чтобы выбрать сторону. Нажмите 'H', чтобы переключить форму сетки.".to_string(),
            "instructions_showing" => "Нажмите Enter или Esc, чтобы сгенерировать новые пресеты. Нажмите 'H', чтобы переключить форму сетки.".to_string(),
            "bird" => "Птица".to_string(),
            "pork" => "Свинья".to_string(),
            "level_format" => format!("Уровень {} (Мир {})", 1, 1), // Placeholder
            "grid_shape_current" => "Квадрат".to_string(),
            "grid_shape_hexagon" => "Этапы".to_string(),
            "grid_shape_toggle" => "Нажмите 'J', чтобы сменить язык".to_string(),
            "language_indicator" => format!("Язык: {}", "RU"), // Placeholder
            _ => key.to_string(),
        }
    }

    fn get_chinese_text(&self, key: &str) -> String {
        match key {
            "title" => "愤怒的小鸟星球大战2 乱斗模式预设生成器".to_string(),
            "bird_side" => "1. 小鸟方".to_string(),
            "pork_side" => "2. 猪猪方".to_string(),
            "choose_side" => "选择阵营".to_string(),
            "instructions_selecting" => "按1或2选择阵营。按'H'切换网格形状。".to_string(),
            "instructions_showing" => "按Enter或Esc生成新预设。按'H'切换网格形状。".to_string(),
            "bird" => "小鸟".to_string(),
            "pork" => "小猪".to_string(),
            "level_format" => format!("关卡 {} (世界 {})", 1, 1), // Placeholder
            "grid_shape_current" => "方形".to_string(),
            "grid_shape_hexagon" => "阶段".to_string(),
            "grid_shape_toggle" => "按'J'切换语言".to_string(),
            "language_indicator" => format!("语言: {}", "ZH"), // Placeholder
            _ => key.to_string(),
        }
    }

    fn get_japanese_text(&self, key: &str) -> String {
        match key {
            "title" => " Angry Birds Star Wars 2 ローグライク プリセット ジェネレーター".to_string(),
            "bird_side" => "1. 鳥側".to_string(),
            "pork_side" => "2. 豚側".to_string(),
            "choose_side" => "サイドを選択".to_string(),
            "instructions_selecting" => "1または2を押してサイドを選択してください。'H'を押してグリッド形状を切り替えます。".to_string(),
            "instructions_showing" => "EnterまたはEscを押して新しいプリセットを生成します。'H'を押してグリッド形状を切り替えます。".to_string(),
            "bird" => "鳥".to_string(),
            "pork" => "豚".to_string(),
            "level_format" => format!("レベル {} (ワールド {})", 1, 1), // Placeholder
            "grid_shape_current" => "四角".to_string(),
            "grid_shape_hexagon" => "ステージ".to_string(),
            "grid_shape_toggle" => "'J'を押して言語を変更".to_string(),
            "language_indicator" => format!("言語: {}", "JA"), // Placeholder
            _ => key.to_string(),
        }
    }

    fn get_korean_text(&self, key: &str) -> String {
        match key {
            "title" => "앵그리 버드 스타워즈 2 로그라이크 프리셋 생성기".to_string(),
            "bird_side" => "1. 새 측".to_string(),
            "pork_side" => "2. 돼지 측".to_string(),
            "choose_side" => "측면 선택".to_string(),
            "instructions_selecting" => "1 또는 2를 눌러 측면을 선택하세요. 'H'를 눌러 격자 모양을 전환합니다.".to_string(),
            "instructions_showing" => "Enter 또는 Esc를 눌러 새 프리셋을 생성하세요. 'H'를 눌러 격자 모양을 전환합니다.".to_string(),
            "bird" => "새".to_string(),
            "pork" => "돼지".to_string(),
            "level_format" => format!("레벨 {} (월드 {})", 1, 1), // Placeholder
            "grid_shape_current" => "사각형".to_string(),
            "grid_shape_hexagon" => "단계".to_string(),
            "grid_shape_toggle" => "'J'를 눌러 언어 변경".to_string(),
            "language_indicator" => format!("언어: {}", "KO"), // Placeholder
            _ => key.to_string(),
        }
    }

    fn get_arabic_text(&self, key: &str) -> String {
        match key {
            "title" => "مولد الإعدادات الجاهزة لأنغري بيردز ستار وورز 2 نمط الـ Roguelike".to_string(),
            "bird_side" => "1. جانب الطيور".to_string(),
            "pork_side" => "2. جانب الخنازير".to_string(),
            "choose_side" => "اختر الجانب".to_string(),
            "instructions_selecting" => "اضغط 1 أو 2 لاختيار الجانب. اضغط 'H' لتغيير شكل الشبكة.".to_string(),
            "instructions_showing" => "اضغط Enter أو Esc لإنشاء إعدادات جديدة. اضغط 'H' لتغيير شكل الشبكة.".to_string(),
            "bird" => "طائر".to_string(),
            "pork" => "خنزير".to_string(),
            "level_format" => format!("المرحلة {} (العالم {})", 1, 1), // Placeholder
            "grid_shape_current" => "مربع".to_string(),
            "grid_shape_hexagon" => "مراحل".to_string(), // Stages in Arabic
            "grid_shape_toggle" => "اضغط 'J' لتغيير اللغة".to_string(),
            "language_indicator" => format!("اللغة: {}", "AR"), // Placeholder
            _ => key.to_string(),
        }
    }

    fn get_hindi_text(&self, key: &str) -> String {
        match key {
            "title" => "एंग्री बर्ड्स स्टार वॉर्स 2 रॉग्लाइक प्रीसेट जनरेटर".to_string(),
            "bird_side" => "1. पक्षी पक्ष".to_string(),
            "pork_side" => "2. सुअर पक्ष".to_string(),
            "choose_side" => "पक्ष चुनें".to_string(),
            "instructions_selecting" => "पक्ष चुनने के लिए 1 या 2 दबाएं। ग्रिड आकार बदलने के लिए 'H' दबाएं।".to_string(),
            "instructions_showing" => "नए प्रीसेट उत्पन्न करने के लिए Enter या Esc दबाएं। ग्रिड आकार बदलने के लिए 'H' दबाएं।".to_string(),
            "bird" => "पक्षी".to_string(),
            "pork" => "सुअर".to_string(),
            "level_format" => format!("स्तर {} (विश्व {})", 1, 1), // Placeholder
            "grid_shape_current" => "वर्ग".to_string(),
            "grid_shape_hexagon" => "चरण".to_string(), // Stages in Hindi
            "grid_shape_toggle" => "भाषा बदलने के लिए 'J' दबाएं".to_string(),
            "language_indicator" => format!("भाषा: {}", "HI"), // Placeholder
            _ => key.to_string(),
        }
    }

    fn get_turkish_text(&self, key: &str) -> String {
        match key {
            "title" => "Angry Birds Star Wars 2 Roguelike Ön Ayar Üretici".to_string(),
            "bird_side" => "1. Kuş Tarafı".to_string(),
            "pork_side" => "2. Domuz Tarafı".to_string(),
            "choose_side" => "Taraf Seç".to_string(),
            "instructions_selecting" => "Bir taraf seçmek için 1 veya 2 tuşuna basın. Izgara şeklini değiştirmek için 'H' tuşuna basın.".to_string(),
            "instructions_showing" => "Yeni ön ayarlar oluşturmak için Enter veya Esc tuşuna basın. Izgara şeklini değiştirmek için 'H' tuşuna basın.".to_string(),
            "bird" => "Kuş".to_string(),
            "pork" => "Domuz".to_string(),
            "level_format" => format!("Seviye {} (Dünya {})", 1, 1), // Placeholder
            "grid_shape_current" => "Kare".to_string(),
            "grid_shape_hexagon" => "Aşamalar".to_string(),
            "grid_shape_toggle" => "Dili değiştirmek için 'J' tuşuna basın".to_string(),
            "language_indicator" => format!("Dil: {}", "TR"), // Placeholder
            _ => key.to_string(),
        }
    }

    fn get_polish_text(&self, key: &str) -> String {
        match key {
            "title" => "Generator Presetów Angry Birds Star Wars 2 w Stylu Roguelike".to_string(),
            "bird_side" => "1. Strona Ptaków".to_string(),
            "pork_side" => "2. Strona Świń".to_string(),
            "choose_side" => "Wybierz Stronę".to_string(),
            "instructions_selecting" => "Naciśnij 1 lub 2, aby wybrać stronę. Naciśnij 'H', aby przełączyć kształt siatki.".to_string(),
            "instructions_showing" => "Naciśnij Enter lub Esc, aby wygenerować nowe preset'y. Naciśnij 'H', aby przełączyć kształt siatki.".to_string(),
            "bird" => "Ptak".to_string(),
            "pork" => "Świnia".to_string(),
            "level_format" => format!("Poziom {} (Świat {})", 1, 1), // Placeholder
            "grid_shape_current" => "Kwadrat".to_string(),
            "grid_shape_hexagon" => "Etap".to_string(),
            "grid_shape_toggle" => "Naciśnij 'J', aby zmienić język".to_string(),
            "language_indicator" => format!("Język: {}", "PL"), // Placeholder
            _ => key.to_string(),
        }
    }

    fn get_czech_text(&self, key: &str) -> String {
        match key {
            "title" => "Generátor přednastavení Angry Birds Star Wars 2 ve stylu Roguelike".to_string(),
            "bird_side" => "1. Strana ptáků".to_string(),
            "pork_side" => "2. Strana prasat".to_string(),
            "choose_side" => "Vyberte stranu".to_string(),
            "instructions_selecting" => "Stiskněte 1 nebo 2 pro výběr strany. Stiskněte 'H' pro přepnutí tvaru mřížky.".to_string(),
            "instructions_showing" => "Stiskněte Enter nebo Esc pro vygenerování nových přednastavení. Stiskněte 'H' pro přepnutí tvaru mřížky.".to_string(),
            "bird" => "Pták".to_string(),
            "pork" => "Prase".to_string(),
            "level_format" => format!("Úroveň {} (Svět {})", 1, 1), // Placeholder
            "grid_shape_current" => "Čtverec".to_string(),
            "grid_shape_hexagon" => "Stupně".to_string(),
            "grid_shape_toggle" => "Stiskněte 'J' pro změnu jazyka".to_string(),
            "language_indicator" => format!("Jazyk: {}", "CS"), // Placeholder
            _ => key.to_string(),
        }
    }

    fn get_swedish_text(&self, key: &str) -> String {
        match key {
            "title" => "Angry Birds Star Wars 2 Roguelike Förinställningsgenerator".to_string(),
            "bird_side" => "1. Fågelsidan".to_string(),
            "pork_side" => "2. Gritsiden".to_string(),
            "choose_side" => "Välj sida".to_string(),
            "instructions_selecting" => "Tryck 1 eller 2 för att välja en sida. Tryck 'H' för att växla rutnätsformen.".to_string(),
            "instructions_showing" => "Tryck Enter eller Esc för att generera nya förinställningar. Tryck 'H' för att växla rutnätsformen.".to_string(),
            "bird" => "Fågel".to_string(),
            "pork" => "Gris".to_string(),
            "level_format" => format!("Nivå {} (Värld {})", 1, 1), // Placeholder
            "grid_shape_current" => "Kvadrat".to_string(),
            "grid_shape_hexagon" => "Steg".to_string(),
            "grid_shape_toggle" => "Tryck 'J' för att ändra språk".to_string(),
            "language_indicator" => format!("Språk: {}", "SV"), // Placeholder
            _ => key.to_string(),
        }
    }

    fn get_finnish_text(&self, key: &str) -> String {
        match key {
            "title" => "Angry Birds Star Wars 2 Roguelike Esivalmistaja".to_string(),
            "bird_side" => "1. Lintupuoli".to_string(),
            "pork_side" => "2. Possupuoli".to_string(),
            "choose_side" => "Valitse puoli".to_string(),
            "instructions_selecting" => "Paina 1 tai 2 valitaksesi puolen. Paina 'H' vaihtaaksesi ruudukon muotoa.".to_string(),
            "instructions_showing" => "Paina Enter tai Esc generoidaksesi uudet esiasetukset. Paina 'H' vaihtaaksesi ruudukon muotoa.".to_string(),
            "bird" => "Lintu".to_string(),
            "pork" => "Possu".to_string(),
            "level_format" => format!("Taso {} (Maailma {})", 1, 1), // Placeholder
            "grid_shape_current" => "Neliö".to_string(),
            "grid_shape_hexagon" => "Vaiheet".to_string(),
            "grid_shape_toggle" => "Paina 'J' vaihtaaksesi kieltä".to_string(),
            "language_indicator" => format!("Kieli: {}", "FI"), // Placeholder
            _ => key.to_string(),
        }
    }

    fn get_norwegian_text(&self, key: &str) -> String {
        match key {
            "title" => "Angry Birds Star Wars 2 Roguelike Forhåndsinnstillingsgenerator".to_string(),
            "bird_side" => "1. Fuglesiden".to_string(),
            "pork_side" => "2. Svinepå".to_string(),
            "choose_side" => "Velg side".to_string(),
            "instructions_selecting" => "Trykk 1 eller 2 for å velge en side. Trykk 'H' for å veksle rutenettformen.".to_string(),
            "instructions_showing" => "Trykk Enter eller Esc for å generere nye forhåndsinnstillinger. Trykk 'H' for å veksle rutenettformen.".to_string(),
            "bird" => "Fugl".to_string(),
            "pork" => "Gris".to_string(),
            "level_format" => format!("Nivå {} (Verden {})", 1, 1), // Placeholder
            "grid_shape_current" => "Firkant".to_string(),
            "grid_shape_hexagon" => "Trinn".to_string(),
            "grid_shape_toggle" => "Trykk 'J' for å endre språk".to_string(),
            "language_indicator" => format!("Språk: {}", "NO"), // Placeholder
            _ => key.to_string(),
        }
    }

    fn get_danish_text(&self, key: &str) -> String {
        match key {
            "title" => "Angry Birds Star Wars 2 Roguelike Forudindstilling Generator".to_string(),
            "bird_side" => "1. Fuglesiden".to_string(),
            "pork_side" => "2. Grisenes side".to_string(),
            "choose_side" => "Vælg side".to_string(),
            "instructions_selecting" => "Tryk 1 eller 2 for at vælge en side. Tryk 'H' for at skifte gitterform.".to_string(),
            "instructions_showing" => "Tryk Enter eller Esc for at generere nye forudindstillinger. Tryk 'H' for at skifte gitterform.".to_string(),
            "bird" => "Fugl".to_string(),
            "pork" => "Gris".to_string(),
            "level_format" => format!("Niveau {} (Verden {})", 1, 1), // Placeholder
            "grid_shape_current" => "Firkant".to_string(),
            "grid_shape_hexagon" => "Trin".to_string(),
            "grid_shape_toggle" => "Tryk 'J' for at skifte sprog".to_string(),
            "language_indicator" => format!("Sprog: {}", "DA"), // Placeholder
            _ => key.to_string(),
        }
    }
}

// Define the bird and pig character lists
const BIRD_CHARACTERS: &[&str] = &[
    "Ezra Bridger",
    "Sabine Wren",
    "Chopper",
    "Hera Syndulla",
    "Kana Jarrus",
    "Garazeb \"Zeb\" Orrelios",
    "Qui-Gon Jinn",
    "Obi-Wan Kenobi",
    "Yoda",
    "Jar Jar Binks",
    "Anakin E1",
    "Captain Panakin",
    "Padmé",
    "Anakin E2",
    "Jedi Younglings",
    "Made Windu",
    "Chewbacca",
    "R2-D2",
    "C-3PO",
    "Silver C-3PO",
    "Han Solo",
    "Leia",
    "Pilot Luke",
    "Endor Luke",
    "Jedi Luke",
    "Lando",
    "Wicket",
    "Kit Fisto",
    "Carbonite Han Solo",
];

const PIG_CHARACTERS: &[&str] = &[
    "The Inquisitor",
    "Agent Kallus",
    "AT-DP Pilot",
    "Cikatro Vizago",
    "Vizago Droid",
    "Imperial Officer",
    "Jango Fett",
    "Darth Sidious",
    "Battle Droid",
    "Darth Maul",
    "Count Dooku",
    "Droideka",
    "Zam Wesell",
    "General Grievous",
    "Anakin E3",
    "Biker Scout",
    "Stormtrooper",
    "TIE Fighter Pilot",
    "Darth Vader",
    "Boba Fett",
    "Hologram Darth Sidious",
    "Shadowtrooper",
    "Red Battle Droid",
    "Shocktrooper",
    "Jabba",
    "Royal Guard",
    "Tusken Raider",
];

// Custom widget for stages blocks
struct StagesBlock {
    title: String,
    content: Vec<String>,
    style: Style,
}

impl StagesBlock {
    fn new(title: String, content: Vec<String>) -> Self {
        Self {
            title,
            content,
            style: Style::default(),
        }
    }

    fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Widget for StagesBlock {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        // Draw a stages shape using ASCII art style
        let width = area.width as usize;
        let height = area.height as usize;

        if width < 10 || height < 8 {
            // If area is too small, fall back to a simple block
            let fallback_block = Block::default()
                .title(self.title)
                .borders(Borders::ALL)
                .style(self.style);
            fallback_block.render(area, buf);
            return;
        }

        // Clear the area with background
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                buf.get_mut(x, y).set_char(' ')
                    .set_style(self.style);
            }
        }

        // Draw stages border using ASCII art style
        let center_x = area.x + area.width / 2;
        let center_y = area.y + area.height / 2;

        // Calculate stages dimensions based on available space
        let stages_width = (width / 2).min(15); // Limit width for readability
        let stages_height = (height / 2).min(6); // Limit height for readability

        // Draw the stages using ASCII art pattern
        draw_ascii_stages(buf, area, center_x, center_y, stages_width as u16, stages_height as u16, &self.title, &self.content, self.style);
    }
}

// Helper function to draw ASCII-style stages
fn draw_ascii_stages(buf: &mut ratatui::buffer::Buffer, area: Rect, center_x: u16, _center_y: u16, stages_width: u16, stages_height: u16, title: &str, content: &[String], style: Style) {
    let stages_width = stages_width.max(5);
    let stages_height = stages_height.max(3);

    // Top part of stages (upper triangle)
    for i in 0..stages_height {
        let y = area.y + i + 1;
        if y >= area.bottom() { break; }

        // Left side
        let left_x = center_x - (stages_width - i);
        if left_x >= area.left() && left_x < area.right() {
            buf.get_mut(left_x, y).set_char('/')
                .set_style(style.fg(Color::Magenta));
        }

        // Right side
        let right_x = center_x + (stages_width - i);
        if right_x >= area.left() && right_x < area.right() {
            buf.get_mut(right_x, y).set_char('\\')
                .set_style(style.fg(Color::Magenta));
        }
    }

    // Middle part of stages (parallelogram)
    for i in 0..stages_height {
        let y = area.y + stages_height + i;
        if y >= area.bottom() { break; }

        // Left side
        let left_x = area.x + 1;
        if left_x >= area.left() && left_x < area.right() {
            buf.get_mut(left_x, y).set_char('/')
                .set_style(style.fg(Color::Magenta));
        }

        // Right side
        let right_x = area.x + area.width - 2;
        if right_x >= area.left() && right_x < area.right() {
            buf.get_mut(right_x, y).set_char('\\')
                .set_style(style.fg(Color::Magenta));
        }
    }

    // Bottom part of stages (lower triangle)
    for i in 0..stages_height {
        let y = area.y + stages_height * 2 + i;
        if y >= area.bottom() { break; }

        // Left side
        let left_x = area.x + 1 + i;
        if left_x >= area.left() && left_x < area.right() {
            buf.get_mut(left_x, y).set_char('\\')
                .set_style(style.fg(Color::Magenta));
        }

        // Right side
        let right_x = area.x + area.width - 2 - i;
        if right_x >= area.left() && right_x < area.right() {
            buf.get_mut(right_x, y).set_char('/')
                .set_style(style.fg(Color::Magenta));
        }
    }

    // Draw top and bottom horizontal lines
    for i in 0..(stages_width * 2) {
        // Top horizontal line
        let top_x = center_x - stages_width + i;
        if top_x >= area.left() + 1 && top_x < area.right() - 1 && area.y + 1 < area.bottom() {
            buf.get_mut(top_x, area.y + 1).set_char('_')
                .set_style(style.fg(Color::Magenta));
        }

        // Bottom horizontal line
        let bottom_y = area.y + stages_height * 2 - 1;
        if bottom_y < area.bottom() && area.y + stages_height * 2 - 1 >= area.top() {
            let bottom_x = center_x - stages_width + i;
            if bottom_x >= area.left() + 1 && bottom_x < area.right() - 1 {
                buf.get_mut(bottom_x, bottom_y).set_char('_')
                    .set_style(style.fg(Color::Magenta));
            }
        }
    }

    // Add title at the top center
    let title_x = center_x.saturating_sub((title.len() / 2) as u16).max(area.x + 2);
    for (i, ch) in title.chars().enumerate() {
        let x = title_x + i as u16;
        if x < area.right() - 1 && area.y + 1 < area.bottom() {
            buf.get_mut(x, area.y + 1).set_char(ch)
                .set_style(style.fg(Color::Cyan));
        }
    }

    // Add content inside the stages
    for (i, content_line) in content.iter().take(8).enumerate() { // Show up to 8 lines of content
        let content_y = area.y + 3 + i as u16; // Start below title
        if content_y >= area.bottom() - 2 {
            break;
        }

        let content_x = center_x.saturating_sub((content_line.len() / 2) as u16).max(area.x + 2);
        for (j, ch) in content_line.chars().enumerate() {
            let x = content_x + j as u16;
            if x < area.right() - 1 && content_y < area.bottom() - 1 {
                buf.get_mut(x, content_y).set_char(ch)
                    .set_style(style.fg(Color::Green));
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.state {
                AppState::SelectingSide => {
                    match key.code {
                        KeyCode::Char('1') => {
                            app.selected_side = Some(Side::Bird);
                            app.state = AppState::GeneratingPreset;
                            // Generate 8 random level presets
                            if let Some(side) = app.selected_side.clone() {
                                app.level_presets = generate_level_presets(side);
                            }
                            app.state = AppState::ShowingPreset;
                        }
                        KeyCode::Char('2') => {
                            app.selected_side = Some(Side::Pork);
                            app.state = AppState::GeneratingPreset;
                            // Generate 8 random level presets
                            if let Some(side) = app.selected_side.clone() {
                                app.level_presets = generate_level_presets(side);
                            }
                            app.state = AppState::ShowingPreset;
                        }
                        KeyCode::Char('h') | KeyCode::Char('H') => {
                            // Toggle grid shape
                            app.grid_shape = match app.grid_shape {
                                GridShape::Square => GridShape::Stages,
                                GridShape::Stages => GridShape::Square,
                            };
                        }
                        KeyCode::Char('j') | KeyCode::Char('J') => {
                            // Change language
                            app.current_language = app.current_language.next_language();
                        }
                        KeyCode::Esc => return Ok(()),
                        _ => {}
                    }
                }
                AppState::ShowingPreset => {
                    if key.code == KeyCode::Enter || key.code == KeyCode::Esc {
                        // Reset to start over
                        app.state = AppState::SelectingSide;
                        app.selected_side = None;
                        app.level_presets.clear();
                        app.current_input.clear();
                    } else if key.code == KeyCode::Char('h') || key.code == KeyCode::Char('H') {
                        // Toggle grid shape
                        app.grid_shape = match app.grid_shape {
                            GridShape::Square => GridShape::Stages,
                            GridShape::Stages => GridShape::Square,
                        };
                    } else if key.code == KeyCode::Char('j') || key.code == KeyCode::Char('J') {
                        // Change language
                        app.current_language = app.current_language.next_language();
                    }
                }
                _ => {} // Other states don't need key handling
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    // Title
    let title = Paragraph::new(app.get_translation("title", &app.current_language))
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(title, chunks[0]);

    // Main content area
    match app.state {
        AppState::SelectingSide => {
            let items = vec![
                ListItem::new(app.get_translation("bird_side", &app.current_language)),
                ListItem::new(app.get_translation("pork_side", &app.current_language)),
            ];
            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title(app.get_translation("choose_side", &app.current_language)))
                .style(Style::default().fg(Color::White));
            f.render_widget(list, chunks[1]);

            let instructions = Paragraph::new(app.get_translation("instructions_selecting", &app.current_language))
                .style(Style::default().fg(Color::Gray))
                .block(Block::default().borders(Borders::NONE));
            f.render_widget(instructions, chunks[2]);
        }
        AppState::ShowingPreset => {
            if let Some(side) = &app.selected_side {
                let side_str = match side {
                    Side::Bird => app.get_translation("bird", &app.current_language),
                    Side::Pork => app.get_translation("pork", &app.current_language),
                };

                let title_text = format!("8 {} - {} {}",
                    app.get_translation("title", &app.current_language).split(" ").nth(0).unwrap_or("Random"), // Get first word as "Random"
                    side_str,
                    app.get_translation("title", &app.current_language).split(" ").nth(2).unwrap_or("Levels") // Get third word as "Levels"
                );
                let title_para = Paragraph::new(title_text)
                    .style(Style::default().fg(Color::Cyan))
                    .block(Block::default().borders(Borders::NONE));
                f.render_widget(title_para, chunks[0]);

                // Create a 4x2 grid layout
                let grid_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ])
                    .split(chunks[1]);

                let row1_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                    ])
                    .split(grid_chunks[0]);

                let row2_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                    ])
                    .split(grid_chunks[1]);

                let all_chunks = [
                    row1_chunks[0], row1_chunks[1], row1_chunks[2], row1_chunks[3],
                    row2_chunks[0], row2_chunks[1], row2_chunks[2], row2_chunks[3],
                ];

                // Render each level preset in its grid cell
                for (i, chunk) in all_chunks.iter().enumerate() {
                    if i < app.level_presets.len() {
                        let preset = &app.level_presets[i];
                        let level_title = match app.grid_shape {
                            GridShape::Square => format!("{} {} ({} {})",
                                app.get_translation("title", &app.current_language).split(" ").nth(4).unwrap_or("Level"), // Get "Level"
                                preset.level,
                                app.get_translation("title", &app.current_language).split(" ").nth(5).unwrap_or("World"), // Get "World"
                                preset.world),
                            GridShape::Stages => format!("L{}-W{}", preset.level, preset.world),
                        };

                        let content: Vec<String> = preset
                            .characters
                            .iter()
                            .take(8) // Show up to 8 characters per level
                            .map(|character| character.name.clone())
                            .collect();

                        // Create widget based on grid shape
                        match app.grid_shape {
                            GridShape::Square => {
                                let items: Vec<ListItem> = content
                                    .iter()
                                    .map(|text| ListItem::new(text.as_str()))
                                    .collect();

                                let block = Block::default().borders(Borders::ALL).title(level_title.clone());
                                let list = List::new(items)
                                    .block(block)
                                    .style(Style::default().fg(Color::Green));
                                f.render_widget(list, *chunk);
                            },
                            GridShape::Stages => {
                                let stages_block = StagesBlock::new(level_title.clone(), content)
                                    .style(Style::default().fg(Color::White));
                                stages_block.render(*chunk, f.buffer_mut());
                            },
                        }
                    }
                }

                let _grid_shape_text = match app.grid_shape {
                    GridShape::Square => app.get_translation("grid_shape_current", &app.current_language),
                    GridShape::Stages => app.get_translation("grid_shape_hexagon", &app.current_language),
                };

                // Get the appropriate instruction text and append language toggle info
                let base_instruction = app.get_translation("instructions_showing", &app.current_language);
                let language_toggle = app.get_translation("grid_shape_toggle", &app.current_language);

                let instructions = Paragraph::new(format!("{} {}", base_instruction, language_toggle))
                    .style(Style::default().fg(Color::Gray))
                    .block(Block::default().borders(Borders::NONE));
                f.render_widget(instructions, chunks[2]);
            }
        }
        _ => {}
    }
}

fn generate_level_presets(side: Side) -> Vec<LevelPreset> {
    let mut rng = thread_rng();
    let mut presets = Vec::new();

    for _ in 0..8 {
        // Randomly select a world (1-6)
        let world = rng.gen_range(1..=6);

        // Calculate max levels for this world
        let max_levels = match world {
            1..=4 => 20,
            5 => 16,
            6 => 12,
            _ => 20,
        };

        // Randomly select a level within the world's range
        let level = rng.gen_range(1..=max_levels);

        // Generate characters for this level
        let characters = generate_preset_for_level(side.clone());

        presets.push(LevelPreset {
            world,
            level,
            characters,
        });
    }

    presets
}

fn generate_preset_for_level(side: Side) -> Vec<Character> {
    let mut rng = thread_rng();
    let mut preset = Vec::new();

    // Select up to 8 characters from the chosen side
    let characters = match side {
        Side::Bird => BIRD_CHARACTERS,
        Side::Pork => PIG_CHARACTERS,
    };

    // Shuffle the characters and take up to 8
    let mut shuffled = characters.to_vec();
    shuffled.shuffle(&mut rng);

    for i in 0..shuffled.len().min(8) {
        preset.push(Character {
            name: shuffled[i].to_string(),
            side: side.clone(),
        });
    }

    preset
}
