use std::str::FromStr;

use schemars::JsonSchema;
use serde::Serialize;

#[derive(Clone, Serialize, JsonSchema)]
pub enum Country {
	#[serde(rename = "AR")]
	Argentina,
	#[serde(rename = "AU")]
	Australia,
	#[serde(rename = "AT")]
	Austria,
	#[serde(rename = "BE")]
	Belgium,
	#[serde(rename = "BR")]
	Brazil,
	#[serde(rename = "MM")]
	Burma,
	#[serde(rename = "CA")]
	Canada,
	#[serde(rename = "CN")]
	China,
	#[serde(rename = "CO")]
	Colombia,
	#[serde(rename = "CY")]
	Cyprus,
	#[serde(rename = "CZ")]
	CzechRepublic,
	#[serde(rename = "DK")]
	Denmark,
	#[serde(rename = "FI")]
	Finland,
	#[serde(rename = "FR")]
	France,
	#[serde(rename = "DE")]
	Germany,
	#[serde(rename = "GT")]
	Guatemala,
	#[serde(rename = "HU")]
	Hungary,
	#[serde(rename = "IN")]
	India,
	#[serde(rename = "ID")]
	Indonesia,
	#[serde(rename = "IQ")]
	Iraq,
	#[serde(rename = "IE")]
	Ireland,
	#[serde(rename = "IL")]
	Israel,
	#[serde(rename = "IT")]
	Italy,
	#[serde(rename = "JP")]
	Japan,
	#[serde(rename = "LB")]
	Lebanon,
	#[serde(rename = "MY")]
	Malaysia,
	#[serde(rename = "MX")]
	Mexico,
	#[serde(rename = "MN")]
	Mongolia,
	#[serde(rename = "NL")]
	Netherlands,
	#[serde(rename = "NZ")]
	NewZealand,
	#[serde(rename = "NO")]
	Norway,
	#[serde(rename = "PE")]
	Peru,
	#[serde(rename = "PL")]
	Poland,
	#[serde(rename = "PT")]
	Portugal,
	#[serde(rename = "QA")]
	Qatar,
	#[serde(rename = "RU")]
	Russia,
	#[serde(rename = "SG")]
	Singapore,
	#[serde(rename = "ZA")]
	SouthAfrica,
	#[serde(rename = "KR")]
	SouthKorea,
	#[serde(rename = "ES")]
	Spain,
	#[serde(rename = "SE")]
	Sweden,
	#[serde(rename = "CH")]
	Switzerland,
	#[serde(rename = "TW")]
	Taiwan,
	#[serde(rename = "TH")]
	Thailand,
	#[serde(rename = "TR")]
	Turkey,
	#[serde(rename = "UA")]
	Ukraine,
	#[serde(rename = "AE")]
	UnitedArabEmirates,
	#[serde(rename = "GB")]
	UnitedKingdom,
	#[serde(rename = "US")]
	UnitedStates,
	#[serde(rename = "VN")]
	Vietnam,
}

impl Country {
	pub fn from_id(id: &str) -> Option<Self> {
		return id.split_once(".")?.1.parse().ok();
	}
}

impl FromStr for Country {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
		match s {
			"argentina" => Ok(Self::Argentina),
			"australia" => Ok(Self::Australia),
			"austria" => Ok(Self::Austria),
			"belgium" => Ok(Self::Belgium),
			"brazil" => Ok(Self::Brazil),
			"burma" => Ok(Self::Burma),
			"canada" => Ok(Self::Canada),
			"china" => Ok(Self::China),
			"colombia" => Ok(Self::Colombia),
			"cyprus" => Ok(Self::Cyprus),
			"czech" => Ok(Self::CzechRepublic),
			"denmark" => Ok(Self::Denmark),
			"finland" => Ok(Self::Finland),
			"france" => Ok(Self::France),
			"germany" => Ok(Self::Germany),
			"guatemala" => Ok(Self::Guatemala),
			"hungary" => Ok(Self::Hungary),
			"india" => Ok(Self::India),
			"indonesia" => Ok(Self::Indonesia),
			"iraq" => Ok(Self::Iraq),
			"ireland" => Ok(Self::Ireland),
			"israel" => Ok(Self::Israel),
			"italy" => Ok(Self::Italy),
			"japan" => Ok(Self::Japan),
			"lebanon" => Ok(Self::Lebanon),
			"malaysia" => Ok(Self::Malaysia),
			"mexico" => Ok(Self::Mexico),
			"mongolia" => Ok(Self::Mongolia),
			"netherlands" => Ok(Self::Netherlands),
			"newzealand" => Ok(Self::NewZealand),
			"norway" => Ok(Self::Norway),
			"peru" => Ok(Self::Peru),
			"poland" => Ok(Self::Poland),
			"portugal" => Ok(Self::Portugal),
			"qatar" => Ok(Self::Qatar),
			"russia" => Ok(Self::Russia),
			"singapore" => Ok(Self::Singapore),
			"southafrica" => Ok(Self::SouthAfrica),
			"southkorea" => Ok(Self::SouthKorea),
			"spain" => Ok(Self::Spain),
			"sweden" => Ok(Self::Sweden),
			"switzerland" => Ok(Self::Switzerland),
			"taiwan" => Ok(Self::Taiwan),
			"thailand" => Ok(Self::Thailand),
			"turkey" => Ok(Self::Turkey),
			"ukraine" => Ok(Self::Ukraine),
			"uae" => Ok(Self::UnitedArabEmirates),
			"uk" => Ok(Self::UnitedKingdom),
			"usa" => Ok(Self::UnitedStates),
			"vietnam" => Ok(Self::Vietnam),
			_ => Err(anyhow::anyhow!("Invalid country")),
		}
	}
}
