package calc

import (
	"fmt"

	"github.com/zajca/zfaktury/internal/domain"
)

// TaxYearConstants holds tax computation constants for a specific year.
type TaxYearConstants struct {
	ProgressiveThreshold   domain.Amount         // prah pro 23% sazbu (in halere)
	BasicCredit            domain.Amount         // sleva na poplatnika
	SpouseCredit           domain.Amount         // sleva na manzela/ku
	StudentCredit          domain.Amount         // student
	DisabilityCredit1      domain.Amount         // invalidita 1. a 2. stupen
	DisabilityCredit3      domain.Amount         // invalidita 3. stupen
	DisabilityZTPP         domain.Amount         // drzitel prukazu ZTP/P
	ChildBenefit1          domain.Amount         // 1. dite
	ChildBenefit2          domain.Amount         // 2. dite
	ChildBenefit3Plus      domain.Amount         // 3+ dite
	ChildBenefitZTP        domain.Amount         // ZTP prirazka (double)
	SocialMinMonthly       domain.Amount         // min mesicni vym. zaklad CSSZ
	SocialRate             int                   // permille*10, e.g. 292 = 29.2%
	HealthMinMonthly       domain.Amount         // min mesicni vym. zaklad ZP
	HealthRate             int                   // permille*10, e.g. 135 = 13.5%
	FlatRateCaps           map[int]domain.Amount // percent -> max halere amount
	TimeTestYears          int                   // years to hold for time test exemption
	SecurityExemptionLimit domain.Amount         // max exempt amount per year (0 = no limit)

	SpouseIncomeLimit         domain.Amount // max spouse income for credit eligibility
	DeductionCapMortgage      domain.Amount // max deduction for mortgage interest
	DeductionCapLifeInsurance domain.Amount // legacy per-category cap (pre-2024)
	DeductionCapPension       domain.Amount // legacy per-category cap (pre-2024)
	// DeductionCapSavingsCombined is the combined cap across životní pojištění,
	// penzijní spoření a dlouhodobý investiční produkt (DIP) per § 15 odst. 5
	// ZDP účinné od 1. 1. 2024. When non-zero, this supersedes the individual
	// life-insurance/pension caps and ComputeDeductions enforces a single shared
	// pool across all three categories.
	DeductionCapSavingsCombined domain.Amount
	DeductionCapUnionDues       domain.Amount // max deduction for union dues
	MaxChildBonus               domain.Amount // max annual child tax bonus
}

// taxConstantsDB stores year-specific tax constants.
var taxConstantsDB = map[int]TaxYearConstants{
	2024: {
		// 36 × průměrná mzda 43 967 Kč = 1 582 812 Kč.
		ProgressiveThreshold: domain.NewAmount(1_582_812, 0),
		BasicCredit:          domain.NewAmount(30_840, 0),
		SpouseCredit:         domain.NewAmount(24_840, 0),
		// Sleva na studenta (§ 35ba odst. 1 písm. f ZDP) byla zrušena
		// konsolidačním balíčkem (zákon č. 349/2023 Sb.) s účinností od
		// 1. 1. 2024.
		StudentCredit:     domain.NewAmount(0, 0),
		DisabilityCredit1: domain.NewAmount(2_520, 0),
		DisabilityCredit3: domain.NewAmount(5_040, 0),
		DisabilityZTPP:    domain.NewAmount(16_140, 0),
		ChildBenefit1:     domain.NewAmount(15_204, 0),
		ChildBenefit2:     domain.NewAmount(22_320, 0),
		ChildBenefit3Plus: domain.NewAmount(27_840, 0),
		ChildBenefitZTP:   domain.NewAmount(0, 0), // doubled automatically
		// SP min měsíční vyměřovací základ HV = 30 % průměrné mzdy
		// (43 967 Kč × 0,30 = 13 191 Kč). Konsolidační balíček zvedl
		// procento z 25 % na 30 % od 1. 1. 2024.
		SocialMinMonthly: domain.NewAmount(13_191, 0),
		SocialRate:       292,
		// ZP min měsíční vyměřovací základ = 50 % průměrné mzdy
		// (43 967 Kč × 0,50 = 21 983,50 Kč, zaokr. nahoru 21 984 Kč;
		// roční minimum 12 × 21 984 = 263 808 Kč, min záloha 2 968 Kč).
		HealthMinMonthly: domain.NewAmount(21_984, 0),
		HealthRate:       135,
		FlatRateCaps: map[int]domain.Amount{
			80: domain.NewAmount(1_600_000, 0),
			60: domain.NewAmount(1_200_000, 0),
			40: domain.NewAmount(800_000, 0),
			30: domain.NewAmount(600_000, 0),
		},
		TimeTestYears:          3,
		SecurityExemptionLimit: domain.NewAmount(0, 0), // no limit before 2025

		SpouseIncomeLimit:           domain.NewAmount(68_000, 0),
		DeductionCapMortgage:        domain.NewAmount(150_000, 0),
		DeductionCapLifeInsurance:   domain.NewAmount(0, 0), // superseded by combined cap
		DeductionCapPension:         domain.NewAmount(0, 0), // superseded by combined cap
		DeductionCapSavingsCombined: domain.NewAmount(48_000, 0),
		DeductionCapUnionDues:       domain.NewAmount(3_000, 0),
		MaxChildBonus:               domain.NewAmount(60_300, 0),
	},
	2025: {
		// 36 × průměrná mzda 46 557 Kč (nařízení vlády 282/2024 Sb.) = 1 676 052 Kč.
		ProgressiveThreshold: domain.NewAmount(1_676_052, 0),
		BasicCredit:          domain.NewAmount(30_840, 0),
		SpouseCredit:         domain.NewAmount(24_840, 0),
		StudentCredit:        domain.NewAmount(0, 0), // zrušena od 2024
		DisabilityCredit1:    domain.NewAmount(2_520, 0),
		DisabilityCredit3:    domain.NewAmount(5_040, 0),
		DisabilityZTPP:       domain.NewAmount(16_140, 0),
		ChildBenefit1:        domain.NewAmount(15_204, 0),
		ChildBenefit2:        domain.NewAmount(22_320, 0),
		ChildBenefit3Plus:    domain.NewAmount(27_840, 0),
		ChildBenefitZTP:      domain.NewAmount(0, 0),
		// 35 % × 46 557 = 16 294,95 → 16 295.
		SocialMinMonthly: domain.NewAmount(16_295, 0),
		SocialRate:       292,
		// 50 % × 46 557 = 23 278,50 → 23 279.
		HealthMinMonthly: domain.NewAmount(23_279, 0),
		HealthRate:       135,
		FlatRateCaps: map[int]domain.Amount{
			80: domain.NewAmount(1_600_000, 0),
			60: domain.NewAmount(1_200_000, 0),
			40: domain.NewAmount(800_000, 0),
			30: domain.NewAmount(600_000, 0),
		},
		TimeTestYears:          3,
		SecurityExemptionLimit: domain.NewAmount(100_000_000, 0), // 1M CZK

		SpouseIncomeLimit:           domain.NewAmount(68_000, 0),
		DeductionCapMortgage:        domain.NewAmount(150_000, 0),
		DeductionCapLifeInsurance:   domain.NewAmount(0, 0),
		DeductionCapPension:         domain.NewAmount(0, 0),
		DeductionCapSavingsCombined: domain.NewAmount(48_000, 0),
		DeductionCapUnionDues:       domain.NewAmount(3_000, 0),
		MaxChildBonus:               domain.NewAmount(60_300, 0),
	},
	2026: {
		// TODO: update once nařízení vlády stanovující průměrnou mzdu pro rok
		// 2026 vyjde — limit = 36 × průměrná mzda. Hodnoty níže jsou placeholdery
		// (sociální minimum: 40 % × průměrná mzda; zdravotní: 50 %; progresivní
		// práh: 36 ×). Přepište před filing season 2027.
		ProgressiveThreshold: domain.NewAmount(1_582_812, 0),
		BasicCredit:          domain.NewAmount(30_840, 0),
		SpouseCredit:         domain.NewAmount(24_840, 0),
		StudentCredit:        domain.NewAmount(0, 0), // zrušena od 2024
		DisabilityCredit1:    domain.NewAmount(2_520, 0),
		DisabilityCredit3:    domain.NewAmount(5_040, 0),
		DisabilityZTPP:       domain.NewAmount(16_140, 0),
		ChildBenefit1:        domain.NewAmount(15_204, 0),
		ChildBenefit2:        domain.NewAmount(22_320, 0),
		ChildBenefit3Plus:    domain.NewAmount(27_840, 0),
		ChildBenefitZTP:      domain.NewAmount(0, 0),
		SocialMinMonthly:     domain.NewAmount(12_139, 0), // PLACEHOLDER
		SocialRate:           292,
		HealthMinMonthly:     domain.NewAmount(11_396, 0), // PLACEHOLDER
		HealthRate:           135,
		FlatRateCaps: map[int]domain.Amount{
			80: domain.NewAmount(1_600_000, 0),
			60: domain.NewAmount(1_200_000, 0),
			40: domain.NewAmount(800_000, 0),
			30: domain.NewAmount(600_000, 0),
		},
		TimeTestYears:          3,
		SecurityExemptionLimit: domain.NewAmount(100_000_000, 0), // 1M CZK

		SpouseIncomeLimit:           domain.NewAmount(68_000, 0),
		DeductionCapMortgage:        domain.NewAmount(150_000, 0),
		DeductionCapLifeInsurance:   domain.NewAmount(0, 0),
		DeductionCapPension:         domain.NewAmount(0, 0),
		DeductionCapSavingsCombined: domain.NewAmount(48_000, 0),
		DeductionCapUnionDues:       domain.NewAmount(3_000, 0),
		MaxChildBonus:               domain.NewAmount(60_300, 0),
	},
}

// GetTaxConstants returns the tax constants for a given year.
func GetTaxConstants(year int) (TaxYearConstants, error) {
	c, ok := taxConstantsDB[year]
	if !ok {
		return TaxYearConstants{}, fmt.Errorf("no tax constants for year %d: %w", year, domain.ErrInvalidInput)
	}
	return c, nil
}
