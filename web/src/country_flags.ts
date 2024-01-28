export type CountryName =
	| 'argentina'
	| 'australia'
	| 'austria'
	| 'belgium'
	| 'brazil'
	| 'burma'
	| 'canada'
	| 'china'
	| 'colombia'
	| 'cyprus'
	| 'czech'
	| 'denmark'
	| 'finland'
	| 'france'
	| 'germany'
	| 'guatemala'
	| 'hungary'
	| 'india'
	| 'indonesia'
	| 'iraq'
	| 'ireland'
	| 'israel'
	| 'italy'
	| 'japan'
	| 'lebanon'
	| 'malaysia'
	| 'mexico'
	| 'mongolia'
	| 'netherlands'
	| 'newzealand'
	| 'norway'
	| 'peru'
	| 'poland'
	| 'portugal'
	| 'qatar'
	| 'russia'
	| 'singapore'
	| 'southafrica'
	| 'southkorea'
	| 'spain'
	| 'sweden'
	| 'switzerland'
	| 'taiwan'
	| 'thailand'
	| 'turkey'
	| 'ukraine'
	| 'uae'
	| 'uk'
	| 'usa'
	| 'vietnam';

export type CountryCode =
	| 'AR'
	| 'AU'
	| 'AT'
	| 'BE'
	| 'BR'
	| 'MM'
	| 'CA'
	| 'CN'
	| 'CO'
	| 'CY'
	| 'CZ'
	| 'DK'
	| 'FI'
	| 'FR'
	| 'DE'
	| 'GT'
	| 'HU'
	| 'IN'
	| 'ID'
	| 'IQ'
	| 'IE'
	| 'IL'
	| 'IT'
	| 'JP'
	| 'LB'
	| 'MY'
	| 'MX'
	| 'MN'
	| 'NL'
	| 'NZ'
	| 'NO'
	| 'PE'
	| 'PL'
	| 'PT'
	| 'QA'
	| 'RU'
	| 'SG'
	| 'ZA'
	| 'KR'
	| 'ES'
	| 'SE'
	| 'CH'
	| 'TW'
	| 'TH'
	| 'TR'
	| 'UA'
	| 'AE'
	| 'GB'
	| 'US'
	| 'VN';

const nameToCountryCode = new Map<CountryName, CountryCode>([
	['argentina', 'AR'],
	['australia', 'AU'],
	['austria', 'AT'],
	['belgium', 'BE'],
	['brazil', 'BR'],
	['burma', 'MM'],
	['canada', 'CA'],
	['china', 'CN'],
	['colombia', 'CO'],
	['cyprus', 'CY'],
	['czech', 'CZ'],
	['denmark', 'DK'],
	['finland', 'FI'],
	['france', 'FR'],
	['germany', 'DE'],
	['guatemala', 'GT'],
	['hungary', 'HU'],
	['india', 'IN'],
	['indonesia', 'ID'],
	['iraq', 'IQ'],
	['ireland', 'IE'],
	['israel', 'IL'],
	['italy', 'IT'],
	['japan', 'JP'],
	['lebanon', 'LB'],
	['malaysia', 'MY'],
	['mexico', 'MX'],
	['mongolia', 'MN'],
	['netherlands', 'NL'],
	['newzealand', 'NZ'],
	['norway', 'NO'],
	['peru', 'PE'],
	['poland', 'PL'],
	['portugal', 'PT'],
	['qatar', 'QA'],
	['russia', 'RU'],
	['singapore', 'SG'],
	['southafrica', 'ZA'],
	['southkorea', 'KR'],
	['spain', 'ES'],
	['sweden', 'SE'],
	['switzerland', 'CH'],
	['taiwan', 'TW'],
	['thailand', 'TH'],
	['turkey', 'TR'],
	['ukraine', 'UA'],
	['uae', 'AE'],
	['uk', 'GB'],
	['usa', 'US'],
	['vietnam', 'VN'],
]);

const COUNTRY_FLAG_ICONS_URL = 'http://purecatamphetamine.github.io/country-flag-icons/3x2/';

export function getCountryFlagUrl(countryName: CountryName): string {
	return `${COUNTRY_FLAG_ICONS_URL}${nameToCountryCode.get(countryName)}.svg`;
}
