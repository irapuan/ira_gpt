// Estrutura para representar um jogador com notas em cada posição
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Player {
    pub name: String,
    pub qualidade_goleiro: i32,
    pub qualidade_zagueiro: i32,
    pub qualidade_meio: i32,
    pub qualidade_atacante: i32,
    pub speed: i32,
    pub stamina: i32,
}

pub type Team = Vec<Player>;

pub type ListOfPlayers = Vec<Player>;

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl PartialEq<Player> for Player {
    fn eq(&self, other: &Player) -> bool {
        self.name == other.name
    }
}

impl Player {
    pub fn qualidades(&self) -> Vec<i32> {
        vec![
            self.qualidade_goleiro,
            self.qualidade_zagueiro,
            self.qualidade_meio,
            self.qualidade_atacante,
            self.speed,
            self.stamina,
        ]
    }
    pub fn media_qualidade_jogador(&self) -> f32 {
        (self.qualidade_goleiro+self.qualidade_zagueiro+self.qualidade_meio+self.qualidade_atacante+self.speed+self.stamina) as f32/6.0
    }
}

pub enum Criteria {
    Keeper,
    Defender,
    Midfielder,
    Forward,
    Speed,
    Stamina,
}

impl std::fmt::Display for Criteria {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Criteria::Keeper => write!(f, "Goleiro"),
            Criteria::Defender => write!(f, "Zagueiro"),
            Criteria::Midfielder => write!(f, "Meio"),
            Criteria::Forward => write!(f, "Atacante"),
            Criteria::Speed => write!(f, "Velocidade"),
            Criteria::Stamina => write!(f, "Stamina"),
        }
    }
}

pub fn rate_average(time: &Team, position: &Criteria) -> f32 {
    let soma_qualidade : i32 = match position {
        Criteria::Keeper => time.iter().filter(|a| a.qualidade_goleiro > 0).map(|j| j.qualidade_goleiro).sum(),
        Criteria::Defender => time.iter().filter(|a| a.qualidade_zagueiro > 0).map(|j| j.qualidade_zagueiro).sum(),
        Criteria::Midfielder => time.iter().filter(|a| a.qualidade_meio > 0).map(|j| j.qualidade_meio).sum(),
        Criteria::Forward => time.iter().filter(|a| a.qualidade_atacante > 0).map(|j| j.qualidade_atacante).sum(),
        Criteria::Speed => time.iter().filter(|a| a.speed > 0).map(|j| j.speed).sum(),
        Criteria::Stamina => time.iter().filter(|a| a.stamina > 0).map(|j| j.stamina).sum(),
    };
    soma_qualidade as f32 / time.len() as f32
}

pub fn rate_max(team: &Team, position: &Criteria) -> i32 {
    match position {
        Criteria::Keeper => team.iter().map(|j| j.qualidade_goleiro).max().unwrap_or(0),
        Criteria::Defender => team.iter().map(|j| j.qualidade_zagueiro).max().unwrap_or(0),
        Criteria::Midfielder => team.iter().map(|j| j.qualidade_meio).max().unwrap_or(0),
        Criteria::Forward => team.iter().map(|j| j.qualidade_atacante).max().unwrap_or(0),
        Criteria::Speed => team.iter().map(|j| j.speed).max().unwrap_or(0),
        Criteria::Stamina => team.iter().map(|j| j.stamina).max().unwrap_or(0),
    }
}


pub fn media_do_jogadores(time: &Team) -> f32 {
    time.iter().map(|player| player.media_qualidade_jogador()).sum::<f32>() / time.len() as f32
}


// Função para calcular a diferença total entre as somas das qualidades dos três times em todas as posições
pub fn total_diference(times: &Vec<Team>) -> f32 {
    let positions = vec![
        Criteria::Keeper,
        Criteria::Defender,
        Criteria::Midfielder,
        Criteria::Forward,
        Criteria::Speed,
        Criteria::Stamina,
    ];
    let mut diff = 0.0;

    for position in positions {
        let sums: Vec<f32> = times.iter().map(|time| rate_average(time, &position)).collect();
        let max_sum = sums.iter().cloned().fold(f32::NEG_INFINITY, |a,b| a.max(b));
        let min_sum = sums.iter().cloned().fold(f32::INFINITY, |a,b| a.min(b));
        println!("criteria {}, max: {}, min:{}", position, max_sum, min_sum);
        diff += max_sum - min_sum;
    }

    diff
}
