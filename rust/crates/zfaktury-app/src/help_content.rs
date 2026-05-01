/// Help content data module -- all 70 topics ported from SvelteKit help-content.ts.
/// Pure data, no GPUI dependency.
use zfaktury_core::calc::constants::TaxYearConstants;
use zfaktury_domain::Amount;

/// Identifies a help topic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum HelpTopicId {
    VariabilniSymbol,
    KonstantniSymbol,
    Duzp,
    DatumSplatnosti,
    ZpusobPlatby,
    PoznamkaFaktura,
    PoznamkaInterni,
    QrPlatba,
    DanoveUznatelny,
    PodilPodnikani,
    SazbaDph,
    CisloDokladu,
    Ico,
    Dic,
    Ares,
    Iban,
    SwiftBic,
    PlatceDph,
    PriznaniDph,
    KontrolniHlaseni,
    SouhrnneHlaseni,
    TypPodani,
    CiselneRady,
    PrefixFormat,
    PrijmyNaklady,
    NeuhrazeneFaktury,
    FakturyPoSplatnosti,
    FrekvenceOpakovani,
    VystupniDph,
    VstupniDph,
    PreneseniDanovePovinnosti,
    NadmernyOdpocet,
    ZakladDane,
    SekceKontrolniHlaseni,
    Dppd,
    KodPlneni,
    ZdanovaciObdobi,
    TypFaktury,
    Dobropis,
    VyrovnaniZalohy,
    IsdocExport,
    DanovaKontrola,
    OcrImport,
    PlatebniPodminky,
    EmailSablony,
    OpakovaneFaktury,
    KategorieNakladu,
    DuplikaceFaktury,
    RocniDane,
    PausalniVydaje,
    Dan1523,
    VymeroviciZaklad,
    CasovyTest,
    SlevaNaPoplatnika,
    ZvyhodneniNaDeti,
    MesiceProporcializace,
    NezdanitelneOdpocty,
    PrehledCssz,
    PrehledZp,
    KapitalovePrijmyS8,
    ObchodyCpS10,
    NutnoPriznatDp,
    DoplatekPreplatek,
    SrazenaDan,
    KurzCnb,
    NovaZaloha,
    FifoPrepocet,
    SlevaNaManzela,
    Ztpp,
}

/// A help topic with title, simple explanation, and legal reference.
pub struct HelpTopic {
    pub title: &'static str,
    pub simple: String,
    pub legal: &'static str,
}

fn fmt_czk(amount: Amount) -> String {
    let czk = amount.to_czk();
    let whole = czk as i64;
    // Format with Czech thousands separator
    let s = whole.to_string();
    let mut result = String::new();
    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.insert(0, '\u{a0}'); // non-breaking space
        }
        result.insert(0, ch);
    }
    format!("{result}\u{a0}K\u{010d}")
}

/// Returns the help topic for the given ID, optionally interpolating
/// year-specific tax constants for dynamic topics.
pub fn get_help_topic(id: HelpTopicId, tc: Option<&TaxYearConstants>) -> HelpTopic {
    match id {
        HelpTopicId::VariabilniSymbol => HelpTopic {
            title: "Variabiln\u{00ed} symbol",
            simple: "Variabiln\u{00ed} symbol je \u{010d}\u{00ed}slo, kter\u{00e9} identifikuje platbu. Kdy\u{017e} v\u{00e1}m n\u{011b}kdo po\u{0161}le pen\u{00ed}ze na \u{00fa}\u{010d}et, banka podle variabiln\u{00ed}ho symbolu pozn\u{00e1}, ke kter\u{00e9} faktu\u{0159}e platba pat\u{0159}\u{00ed}.\n\nV\u{011b}t\u{0161}inou se pou\u{017e}\u{00ed}v\u{00e1} \u{010d}\u{00ed}slo faktury nebo jeho \u{010d}\u{00e1}st. D\u{016f}le\u{017e}it\u{00e9} je, aby ka\u{017e}d\u{00e1} faktura m\u{011b}la unik\u{00e1}tn\u{00ed} variabiln\u{00ed} symbol -- jinak nepozn\u{00e1}te, kdo za co platil.".into(),
            legal: "Variabiln\u{00ed} symbol je numerick\u{00e9} pole o maxim\u{00e1}ln\u{00ed} d\u{00e9}lce 10 \u{010d}\u{00ed}slic. Je definov\u{00e1}n vyhl\u{00e1}\u{0161}kou \u{010c}NB \u{010d}. 169/2011 Sb. jako identifik\u{00e1}tor transakce v tuzemsk\u{00e9}m platebn\u{00ed}m styku.\n\nPodle z\u{00e1}kona \u{010d}. 284/2009 Sb. o platebn\u{00ed}m styku je variabiln\u{00ed} symbol sou\u{010d}\u{00e1}st platebn\u{00ed}ho p\u{0159}\u{00ed}kazu a slou\u{017e}\u{00ed} k identifikaci platby mezi pl\u{00e1}tcem a p\u{0159}\u{00ed}jemcem. Nen\u{00ed} povinn\u{00fd} ze z\u{00e1}kona, ale je standardn\u{00ed} sou\u{010d}\u{00e1}st\u{00ed} faktura\u{010d}n\u{00ed} praxe v \u{010c}R.",
        },
        HelpTopicId::KonstantniSymbol => HelpTopic {
            title: "Konstantn\u{00ed} symbol",
            simple: "Konstantn\u{00ed} symbol je \u{010d}\u{00ed}slo, kter\u{00e9} \u{0159}\u{00ed}k\u{00e1}, o jak\u{00fd} typ platby se jedn\u{00e1} (nap\u{0159}. platba za zbo\u{017e}\u{00ed}, slu\u{017e}by, n\u{00e1}jem). V praxi se dnes pou\u{017e}\u{00ed}v\u{00e1} minim\u{00e1}ln\u{011b} -- v\u{011b}t\u{0161}ina bank ho nevy\u{017e}aduje a pro OSV\u{010c} nen\u{00ed} pot\u{0159}ebn\u{00fd}.\n\nPokud si nejste jisti, m\u{016f}\u{017e}ete pole nechat pr\u{00e1}zdn\u{00e9}.".into(),
            legal: "Konstantn\u{00ed} symbol je definov\u{00e1}n vyhl\u{00e1}\u{0161}kou \u{010c}NB \u{010d}. 169/2011 Sb. Jedn\u{00e1} se o \u{010d}ty\u{0159}\u{010d}\u{00ed}seln\u{00fd} k\u{00f3}d charakterizuj\u{00ed}c\u{00ed} platbu z hlediska jej\u{00ed}ho \u{00fa}\u{010d}elu. Od roku 2004 nen\u{00ed} jeho uv\u{00e1}d\u{011b}n\u{00ed} povinn\u{00e9} pro b\u{011b}\u{017e}n\u{00e9} platby.\n\nNej\u{010d}ast\u{011b}j\u{0161}\u{00ed} hodnoty: 0008 (platba za zbo\u{017e}\u{00ed}), 0308 (platba za slu\u{017e}by), 0558 (ostatn\u{00ed} bezhotovostn\u{00ed} platby).",
        },
        HelpTopicId::Duzp => HelpTopic {
            title: "Datum uskute\u{010d}n\u{011b}n\u{00ed} zdaniteln\u{00e9}ho pln\u{011b}n\u{00ed} (DUZP)",
            simple: "DUZP je datum, kdy skute\u{010d}n\u{011b} do\u{0161}lo k dod\u{00e1}n\u{00ed} zbo\u{017e}\u{00ed} nebo poskytnut\u{00ed} slu\u{017e}by. Ne kdy jste vystavili fakturu, ne kdy v\u{00e1}m p\u{0159}i\u{0161}ly pen\u{00ed}ze -- ale kdy jste re\u{00e1}ln\u{011b} odvedli pr\u{00e1}ci nebo dodali produkt.\n\nNap\u{0159}. pokud jste programovali web cel\u{00fd} leden a fakturujete a\u{017e} 5. \u{00fa}nora, DUZP bude posledn\u{00ed} den, kdy jste pr\u{00e1}ci p\u{0159}edali (t\u{0159}eba 31. ledna).\n\nPro pl\u{00e1}tce DPH je DUZP kl\u{00ed}\u{010d}ov\u{00e9}, proto\u{017e}e ur\u{010d}uje, do kter\u{00e9}ho zda\u{0148}ovac\u{00ed}ho obdob\u{00ed} faktura pat\u{0159}\u{00ed}.".into(),
            legal: "DUZP je definov\u{00e1}no v z\u{00e1}kon\u{011b} \u{010d}. 235/2004 Sb. o DPH, \u{00a7} 21. U dod\u{00e1}n\u{00ed} zbo\u{017e}\u{00ed} je to den dod\u{00e1}n\u{00ed} (\u{00a7} 21 odst. 1). U poskytov\u{00e1}n\u{00ed} slu\u{017e}eb den poskytnut\u{00ed} nebo den vystaven\u{00ed} da\u{0148}ov\u{00e9}ho dokladu, pokud nastal d\u{0159}\u{00ed}ve (\u{00a7} 21 odst. 3).\n\nPl\u{00e1}tce DPH je povinen p\u{0159}iznat da\u{0148} na v\u{00fd}stupu ke dni uskute\u{010d}n\u{011b}n\u{00ed} zdaniteln\u{00e9}ho pln\u{011b}n\u{00ed} (\u{00a7} 20a). DUZP ur\u{010d}uje zda\u{0148}ovac\u{00ed} obdob\u{00ed}, ve kter\u{00e9}m mus\u{00ed} b\u{00fd}t da\u{0148} odvedena.",
        },
        HelpTopicId::DatumSplatnosti => HelpTopic {
            title: "Datum splatnosti",
            simple: "Datum splatnosti je den, do kter\u{00e9}ho m\u{00e1} odb\u{011b}ratel zaplatit fakturu. Pokud z\u{00e1}kazn\u{00ed}k nezaplat\u{00ed} do tohoto data, faktura je \"po splatnosti\" a m\u{016f}\u{017e}ete uplat\u{0148}ovat \u{00fa}roky z prodlen\u{00ed}.\n\nB\u{011b}\u{017e}n\u{00e1} splatnost je 14 nebo 30 dn\u{00ed} od data vystaven\u{00ed}. M\u{016f}\u{017e}e b\u{00fd}t i del\u{0161}\u{00ed} -- z\u{00e1}le\u{017e}\u{00ed} na dohod\u{011b} s odb\u{011b}ratelem.".into(),
            legal: "Splatnost je smluvn\u{00ed} ujedn\u{00e1}n\u{00ed} dle z\u{00e1}kona \u{010d}. 89/2012 Sb. (ob\u{010d}ansk\u{00fd} z\u{00e1}kon\u{00ed}k), \u{00a7} 1958-1964. Pokud nen\u{00ed} dohodnuta, je splatnost bez zbyte\u{010d}n\u{00e9}ho odkladu po doru\u{010d}en\u{00ed} faktury.\n\nPodle z\u{00e1}kona \u{010d}. 340/2015 Sb. o registru smluv a \u{00a7} 1963 ob\u{010d}ansk\u{00e9}ho z\u{00e1}kon\u{00ed}ku plat\u{00ed} pro vztahy s ve\u{0159}ejn\u{00fd}m sektorem maxim\u{00e1}ln\u{00ed} splatnost 30 dn\u{00ed}. Pro obchodn\u{00ed} vztahy mezi podnikateli je smluvn\u{00ed} splatnost maxim\u{00e1}ln\u{011b} 60 dn\u{00ed} (\u{00a7} 1963a OZ), pokud to nen\u{00ed} v\u{016f}\u{010d}i v\u{011b}\u{0159}iteli hrub\u{011b} nespravedliv\u{00e9}.",
        },
        HelpTopicId::ZpusobPlatby => HelpTopic {
            title: "Zp\u{016f}sob platby",
            simple: "Zp\u{016f}sob platby ur\u{010d}uje, jak odb\u{011b}ratel zaplat\u{00ed} fakturu. Nej\u{010d}ast\u{011b}ji bankovn\u{00ed}m p\u{0159}evodem -- v tom p\u{0159}\u{00ed}pad\u{011b} faktura obsahuje \u{010d}\u{00ed}slo \u{00fa}\u{010d}tu a variabiln\u{00ed} symbol.\n\nDal\u{0161}\u{00ed} mo\u{017e}nosti jsou hotovost, platba kartou nebo dob\u{00ed}rka. Pro \u{00fa}\u{010d}etn\u{00ed} a da\u{0148}ov\u{00e9} \u{00fa}\u{010d}ely je d\u{016f}le\u{017e}it\u{00e9}, aby zp\u{016f}sob platby odpov\u{00ed}dal realit\u{011b}.".into(),
            legal: "Zp\u{016f}sob platby na faktu\u{0159}e nen\u{00ed} povinnou n\u{00e1}le\u{017e}itost\u{00ed} da\u{0148}ov\u{00e9}ho dokladu dle \u{00a7} 29 z\u{00e1}kona \u{010d}. 235/2004 Sb. o DPH. Jedn\u{00e1} se v\u{0161}ak o b\u{011b}\u{017e}nou obchodn\u{00ed} n\u{00e1}le\u{017e}itost.\n\nPro hotovostn\u{00ed} platby plat\u{00ed} limit 270 000 K\u{010d} dle z\u{00e1}kona \u{010d}. 254/2004 Sb. o omezen\u{00ed} plateb v hotovosti (\u{00a7} 4). Poru\u{0161}en\u{00ed} je spr\u{00e1}vn\u{00ed} delikt s pokutou do 500 000 K\u{010d} pro fyzick\u{00e9} osoby.",
        },
        HelpTopicId::PoznamkaFaktura => HelpTopic {
            title: "Pozn\u{00e1}mka na faktu\u{0159}e",
            simple: "Text, kter\u{00fd} se zobraz\u{00ed} p\u{0159}\u{00ed}mo na faktu\u{0159}e, kterou po\u{0161}lete z\u{00e1}kazn\u{00ed}kovi. M\u{016f}\u{017e}ete sem napsat nap\u{0159}. pod\u{011b}kov\u{00e1}n\u{00ed} za spolupr\u{00e1}ci, informaci o prob\u{00ed}haj\u{00ed}c\u{00ed} akci nebo upozorn\u{011b}n\u{00ed} na zm\u{011b}nu bankovn\u{00ed}ho \u{00fa}\u{010d}tu.\n\nTato pozn\u{00e1}mka je viditeln\u{00e1} pro odb\u{011b}ratele.".into(),
            legal: "Pozn\u{00e1}mka na faktu\u{0159}e nen\u{00ed} povinnou n\u{00e1}le\u{017e}itost\u{00ed} da\u{0148}ov\u{00e9}ho dokladu dle \u{00a7} 29 z\u{00e1}kona \u{010d}. 235/2004 Sb. Pokud v\u{0161}ak slou\u{017e}\u{00ed} jako informace o osvobozen\u{00e9}m pln\u{011b}n\u{00ed}, mus\u{00ed} obsahovat odkaz na p\u{0159}\u{00ed}slu\u{0161}n\u{00e9} ustanoven\u{00ed} z\u{00e1}kona (\u{00a7} 29 odst. 2 p\u{00ed}sm. c).\n\nNap\u{0159}. u osvobozen\u{00fd}ch pln\u{011b}n\u{00ed}: \"Osvobozeno od DPH dle \u{00a7} 51 z\u{00e1}kona \u{010d}. 235/2004 Sb.\"",
        },
        HelpTopicId::PoznamkaInterni => HelpTopic {
            title: "Intern\u{00ed} pozn\u{00e1}mka",
            simple: "Soukrom\u{00e1} pozn\u{00e1}mka, kterou vid\u{00ed}te jen vy. Na faktu\u{0159}e se nezobrazuje. M\u{016f}\u{017e}ete sem napsat cokoliv pro vlastn\u{00ed} evidenci -- nap\u{0159}. \"dohodnuto s Petrem 15.3.\", \"sleva za doporu\u{010d}en\u{00ed}\" apod.".into(),
            legal: "Intern\u{00ed} pozn\u{00e1}mka nem\u{00e1} pr\u{00e1}vn\u{00ed} relevanci a neobjevuje se na \u{017e}\u{00e1}dn\u{00e9}m dokladu. Slou\u{017e}\u{00ed} pouze pro intern\u{00ed} evidenci podnikatele.",
        },
        HelpTopicId::QrPlatba => HelpTopic {
            title: "QR platba",
            simple: "QR k\u{00f3}d na faktu\u{0159}e umo\u{017e}n\u{00ed} odb\u{011b}rateli naskenovat platbu mobilem. Po naskenov\u{00e1}n\u{00ed} se v bankovn\u{00ed} aplikaci automaticky p\u{0159}edvypln\u{00ed} \u{010d}\u{00ed}slo \u{00fa}\u{010d}tu, \u{010d}\u{00e1}stka, variabiln\u{00ed} symbol a dal\u{0161}\u{00ed} \u{00fa}daje.\n\nOdb\u{011b}ratel tak nemus\u{00ed} nic opisovat a platba prob\u{011b}hne bez chyb. QR platba je standard \u{010c}esk\u{00e9} bankovn\u{00ed} asociace.".into(),
            legal: "QR platba (SPD -- Short Payment Descriptor) je standard \u{010c}esk\u{00e9} bankovn\u{00ed} asociace pro mobiln\u{00ed} platby. Form\u{00e1}t je definov\u{00e1}n specifikac\u{00ed} CBA a je podporov\u{00e1}n v\u{0161}emi hlavn\u{00ed}mi bankami v \u{010c}R.\n\nForm\u{00e1}t QR k\u{00f3}du: SPD*1.0*ACC:{IBAN}*AM:{\u{010d}\u{00e1}stka}*CC:CZK*X-VS:{variabiln\u{00ed} symbol}*...",
        },
        HelpTopicId::DanoveUznatelny => HelpTopic {
            title: "Da\u{0148}ov\u{011b} uznateln\u{00fd} n\u{00e1}klad",
            simple: "Da\u{0148}ov\u{011b} uznateln\u{00fd} n\u{00e1}klad je v\u{00fd}daj, kter\u{00fd} si m\u{016f}\u{017e}ete ode\u{010d}\u{00ed}st od p\u{0159}\u{00ed}jm\u{016f} a t\u{00ed}m sn\u{00ed}\u{017e}it da\u{0148} z p\u{0159}\u{00ed}jm\u{016f}. Mus\u{00ed} spl\u{0148}ovat podm\u{00ed}nku: byl vynalo\u{017e}en na dosa\u{017e}en\u{00ed}, zaji\u{0161}t\u{011b}n\u{00ed} a udr\u{017e}en\u{00ed} va\u{0161}ich p\u{0159}\u{00ed}jm\u{016f}.\n\nP\u{0159}\u{00ed}klad: Notebook pro pr\u{00e1}ci = da\u{0148}ov\u{011b} uznateln\u{00fd}. Dovolen\u{00e1} = nen\u{00ed} da\u{0148}ov\u{011b} uznateln\u{00e1}.\n\nPokud pou\u{017e}\u{00ed}v\u{00e1}te skute\u{010d}n\u{00e9} v\u{00fd}daje (ne pau\u{0161}\u{00e1}ln\u{00ed}), je d\u{016f}le\u{017e}it\u{00e9} spr\u{00e1}vn\u{011b} ozna\u{010d}it, kter\u{00e9} n\u{00e1}klady jsou da\u{0148}ov\u{011b} uznateln\u{00e9}.".into(),
            legal: "Da\u{0148}ov\u{011b} uznateln\u{00e9} n\u{00e1}klady jsou definov\u{00e1}ny v \u{00a7} 24 z\u{00e1}kona \u{010d}. 586/1992 Sb. o dan\u{00ed}ch z p\u{0159}\u{00ed}jm\u{016f}. Jsou to v\u{00fd}daje vynalo\u{017e}en\u{00e9} na dosa\u{017e}en\u{00ed}, zaji\u{0161}t\u{011b}n\u{00ed} a udr\u{017e}en\u{00ed} zdaniteln\u{00fd}ch p\u{0159}\u{00ed}jm\u{016f}.\n\nDa\u{0148}ov\u{011b} neuznateln\u{00e9} n\u{00e1}klady vy\u{010d}te \u{00a7} 25 t\u{00e9}ho\u{017e} z\u{00e1}kona (nap\u{0159}. repre, pokuty, pen\u{00e1}le). Prokazuj\u{00ed} se da\u{0148}ov\u{00fd}mi doklady -- podnikatel mus\u{00ed} b\u{00fd}t schopen prok\u{00e1}zat \u{00fa}\u{010d}etn\u{00ed} doklad, \u{00fa}\u{010d}el v\u{00fd}daje a souvislost s podnikatelskou \u{010d}innost\u{00ed}.",
        },
        HelpTopicId::PodilPodnikani => HelpTopic {
            title: "Pod\u{00ed}l pro podnik\u{00e1}n\u{00ed}",
            simple: "N\u{011b}kter\u{00e9} n\u{00e1}klady pou\u{017e}\u{00ed}v\u{00e1}te jak pro podnik\u{00e1}n\u{00ed}, tak pro osobn\u{00ed} \u{00fa}\u{010d}ely. Nap\u{0159}. auto, telefon nebo internet. Pod\u{00ed}l pro podnik\u{00e1}n\u{00ed} ur\u{010d}uje, kolik procent n\u{00e1}klad\u{016f} uplatn\u{00ed}te jako da\u{0148}ov\u{00fd} v\u{00fd}daj.\n\nP\u{0159}\u{00ed}klad: Telefon pou\u{017e}\u{00ed}v\u{00e1}te z 60 % pro pr\u{00e1}ci a z 40 % soukrom\u{011b}. Pod\u{00ed}l pro podnik\u{00e1}n\u{00ed} je 60 % a jako da\u{0148}ov\u{00fd} n\u{00e1}klad si uplatn\u{00ed}te 60 % z ceny.\n\nPom\u{011b}rn\u{011b} je t\u{0159}eba rozd\u{011b}lit i DPH na vstupu, pokud jste pl\u{00e1}tce DPH.".into(),
            legal: "Kr\u{00e1}cen\u{00ed} n\u{00e1}klad\u{016f} u majetku pou\u{017e}\u{00ed}van\u{00e9}ho i pro soukrom\u{00e9} \u{00fa}\u{010d}ely upravuje \u{00a7} 24 odst. 2 p\u{00ed}sm. h) z\u{00e1}kona \u{010d}. 586/1992 Sb. N\u{00e1}klady se uplat\u{0148}uj\u{00ed} v pom\u{011b}rn\u{00e9} v\u{00fd}\u{0161}i odpov\u{00ed}daj\u{00ed}c\u{00ed} rozsahu pou\u{017e}it\u{00ed} pro podnikatelskou \u{010d}innost.\n\nPro DPH plat\u{00ed} n\u{00e1}rok na odpo\u{010d}et v pom\u{011b}rn\u{00e9} v\u{00fd}\u{0161}i dle \u{00a7} 75 z\u{00e1}kona \u{010d}. 235/2004 Sb. Podnikatel je povinen v\u{00e9}st evidenci pou\u{017e}it\u{00ed} majetku pro podnikatelsk\u{00e9} a soukrom\u{00e9} \u{00fa}\u{010d}ely.",
        },
        HelpTopicId::SazbaDph => HelpTopic {
            title: "Sazba DPH",
            simple: "Sazba DPH (da\u{0148} z p\u{0159}idan\u{00e9} hodnoty) ur\u{010d}uje, kolik procent dan\u{011b} se p\u{0159}id\u{00e1} k cen\u{011b} zbo\u{017e}\u{00ed} \u{010d}i slu\u{017e}by. V \u{010c}esku jsou aktu\u{00e1}ln\u{011b} dv\u{011b} sazby:\n\n- 21 % -- z\u{00e1}kladn\u{00ed} sazba (v\u{011b}t\u{0161}ina zbo\u{017e}\u{00ed} a slu\u{017e}eb)\n- 12 % -- sn\u{00ed}\u{017e}en\u{00e1} sazba (potraviny, l\u{00e9}ky, knihy, ubytov\u{00e1}n\u{00ed}, stavebn\u{00ed} pr\u{00e1}ce)\n\nPokud nejste pl\u{00e1}tce DPH, DPH ne\u{00fa}\u{010d}tujete a na faktu\u{0159}e uvedete 0 %.".into(),
            legal: "Sazby DPH stanovuje \u{00a7} 47 z\u{00e1}kona \u{010d}. 235/2004 Sb. o DPH. Od 1. 1. 2024 plat\u{00ed} dv\u{011b} sazby: z\u{00e1}kladn\u{00ed} 21 % a sn\u{00ed}\u{017e}en\u{00e1} 12 % (slou\u{010d}en\u{00ed} p\u{016f}vodn\u{00ed}ch dvou sn\u{00ed}\u{017e}en\u{00fd}ch sazeb 15 % a 10 %).\n\nZa\u{0159}azen\u{00ed} zbo\u{017e}\u{00ed} a slu\u{017e}eb do sn\u{00ed}\u{017e}en\u{00e9} sazby je v p\u{0159}\u{00ed}loze \u{010d}. 2, 3 a 3a t\u{00e9}ho\u{017e} z\u{00e1}kona. Nepl\u{00e1}tce DPH nen\u{00ed} opr\u{00e1}vn\u{011b}n vy\u{00fa}\u{010d}tovat da\u{0148} a nesm\u{00ed} ji uv\u{00e9}st na dokladu (\u{00a7} 26 odst. 3).",
        },
        HelpTopicId::CisloDokladu => HelpTopic {
            title: "\u{010c}\u{00ed}slo dokladu",
            simple: "\u{010c}\u{00ed}slo dokladu jednozna\u{010d}n\u{011b} identifikuje \u{00fa}\u{010d}etn\u{00ed} doklad (fakturu, \u{00fa}\u{010d}tenku, pokladn\u{00ed} doklad). Slou\u{017e}\u{00ed} pro evidenci -- abyste ka\u{017e}d\u{00fd} n\u{00e1}klad snadno dohledali.\n\nM\u{016f}\u{017e}e to b\u{00fd}t \u{010d}\u{00ed}slo z p\u{0159}ijat\u{00e9} faktury od dodavatele, nebo va\u{0161}e vlastn\u{00ed} \u{010d}\u{00ed}slo, pokud doklad nem\u{00e1}te (nap\u{0159}. pokladn\u{00ed} blok ozna\u{010d}\u{00ed}te \"P-001\").".into(),
            legal: "Po\u{0159}adov\u{00e9} \u{010d}\u{00ed}slo dokladu je povinnou n\u{00e1}le\u{017e}itost\u{00ed} da\u{0148}ov\u{00e9}ho dokladu dle \u{00a7} 29 odst. 1 p\u{00ed}sm. b) z\u{00e1}kona \u{010d}. 235/2004 Sb. Mus\u{00ed} b\u{00fd}t p\u{0159}i\u{0159}azeno v r\u{00e1}mci jedn\u{00e9} \u{010d}i v\u{00ed}ce \u{010d}\u{00ed}seln\u{00fd}ch \u{0159}ad, kter\u{00e9} zaru\u{010d}uj\u{00ed} jeho jednozna\u{010d}nost.\n\nI pro nepl\u{00e1}tce DPH je jednozna\u{010d}n\u{00e1} identifikace dokladu povinnost\u{00ed} dle \u{00a7} 11 z\u{00e1}kona \u{010d}. 563/1991 Sb. o \u{00fa}\u{010d}etnictv\u{00ed}.",
        },
        HelpTopicId::Ico => HelpTopic {
            title: "Identifika\u{010d}n\u{00ed} \u{010d}\u{00ed}slo osoby (I\u{010c}O)",
            simple: "I\u{010c}O je osmim\u{00ed}stn\u{00e9} \u{010d}\u{00ed}slo, kter\u{00e9} dostane ka\u{017e}d\u{00fd} podnikatel nebo firma p\u{0159}i registraci. Slou\u{017e}\u{00ed} k jednozna\u{010d}n\u{00e9} identifikaci -- jako \"rodn\u{00e9} \u{010d}\u{00ed}slo\" pro podnik\u{00e1}n\u{00ed}.\n\nI\u{010c}O se uv\u{00e1}d\u{00ed} na v\u{0161}ech faktur\u{00e1}ch a obchodn\u{00ed}ch dokumentech. Podle I\u{010c}O si m\u{016f}\u{017e}ete ov\u{011b}\u{0159}it odb\u{011b}ratele v obchodn\u{00ed}m rejst\u{0159}\u{00ed}ku nebo registru ARES.".into(),
            legal: "I\u{010c}O je definov\u{00e1}no z\u{00e1}konem \u{010d}. 111/2009 Sb. o z\u{00e1}kladn\u{00ed}ch registrech, \u{00a7} 24-26. P\u{0159}id\u{011b}luje ho registra\u{010d}n\u{00ed} org\u{00e1}n (\u{017e}ivnostensk\u{00fd} \u{00fa}\u{0159}ad, rejst\u{0159}\u{00ed}kov\u{00fd} soud).\n\nPodle \u{00a7} 29 odst. 1 p\u{00ed}sm. a) z\u{00e1}kona \u{010d}. 235/2004 Sb. je I\u{010c}O povinnou n\u{00e1}le\u{017e}itost\u{00ed} da\u{0148}ov\u{00e9}ho dokladu. Povinnost uv\u{00e1}d\u{011b}t I\u{010c}O na obchodn\u{00ed}ch listin\u{00e1}ch plyne tak\u{00e9} z \u{00a7} 435 z\u{00e1}kona \u{010d}. 89/2012 Sb. (ob\u{010d}ansk\u{00fd} z\u{00e1}kon\u{00ed}k).",
        },
        HelpTopicId::Dic => HelpTopic {
            title: "Da\u{0148}ov\u{00e9} identifika\u{010d}n\u{00ed} \u{010d}\u{00ed}slo (DI\u{010c})",
            simple: "DI\u{010c} je \u{010d}\u{00ed}slo, kter\u{00e9} identifikuje pl\u{00e1}tce dan\u{011b}. V \u{010c}esku m\u{00e1} form\u{00e1}t \"CZ\" + I\u{010c}O (nap\u{0159}. CZ12345678). DI\u{010c} dostanete po registraci k DPH u finan\u{010d}n\u{00ed}ho \u{00fa}\u{0159}adu.\n\nPokud nejste pl\u{00e1}tce DPH, DI\u{010c} nemus\u{00ed}te uv\u{00e1}d\u{011b}t. Pokud jste pl\u{00e1}tce, je DI\u{010c} povinn\u{00e9} na ka\u{017e}d\u{00e9} faktu\u{0159}e.".into(),
            legal: "DI\u{010c} je definov\u{00e1}no v \u{00a7} 130 z\u{00e1}kona \u{010d}. 280/2009 Sb. (da\u{0148}ov\u{00fd} \u{0159}\u{00e1}d). Pro \u{00fa}\u{010d}ely DPH je upraveno v \u{00a7} 4a z\u{00e1}kona \u{010d}. 235/2004 Sb. -- u fyzick\u{00fd}ch osob m\u{00e1} form\u{00e1}t \"CZ\" + rodn\u{00e9} \u{010d}\u{00ed}slo, u pr\u{00e1}vnick\u{00fd}ch osob \"CZ\" + I\u{010c}O.\n\nDI\u{010c} je povinnou n\u{00e1}le\u{017e}itost\u{00ed} da\u{0148}ov\u{00e9}ho dokladu dle \u{00a7} 29 odst. 1 p\u{00ed}sm. a) z\u{00e1}kona \u{010d}. 235/2004 Sb. pro pl\u{00e1}tce DPH.",
        },
        HelpTopicId::Ares => HelpTopic {
            title: "ARES -- Administrativn\u{00ed} registr ekonomick\u{00fd}ch subjekt\u{016f}",
            simple: "ARES je ve\u{0159}ejn\u{00fd} registr, kde si m\u{016f}\u{017e}ete ov\u{011b}\u{0159}it \u{00fa}daje o jak\u{00e9}mkoli podnikateli nebo firm\u{011b} v \u{010c}esku. Sta\u{010d}\u{00ed} zadat I\u{010c}O a zjist\u{00ed}te n\u{00e1}zev, s\u{00ed}dlo, pr\u{00e1}vn\u{00ed} formu a dal\u{0161}\u{00ed} informace.\n\nV ZFaktury se ARES pou\u{017e}\u{00ed}v\u{00e1} pro automatick\u{00e9} dopln\u{011b}n\u{00ed} \u{00fa}daj\u{016f} o odb\u{011b}rateli -- zadejte I\u{010c}O a syst\u{00e9}m st\u{00e1}hne jm\u{00e9}no a adresu automaticky.".into(),
            legal: "ARES je informa\u{010d}n\u{00ed} syst\u{00e9}m ve\u{0159}ejn\u{00e9} spr\u{00e1}vy provozovan\u{00fd} Ministerstvem financ\u{00ed} \u{010c}R. Agreguje data z v\u{00ed}ce registr\u{016f}: obchodn\u{00ed}ho rejst\u{0159}\u{00ed}ku, \u{017e}ivnostensk\u{00e9}ho rejst\u{0159}\u{00ed}ku, registru DPH a dal\u{0161}\u{00ed}ch.\n\nP\u{0159}\u{00ed}stup k dat\u{016f}m je bez\u{00fa}platn\u{00fd} a ve\u{0159}ejn\u{00fd} dle z\u{00e1}kona \u{010d}. 106/1999 Sb. o svobodn\u{00e9}m p\u{0159}\u{00ed}stupu k informac\u{00ed}m. API ARES je dostupn\u{00e9} na ares.gov.cz.",
        },
        HelpTopicId::Iban => HelpTopic {
            title: "IBAN -- Mezin\u{00e1}rodn\u{00ed} \u{010d}\u{00ed}slo bankovn\u{00ed}ho \u{00fa}\u{010d}tu",
            simple: "IBAN je mezin\u{00e1}rodn\u{00ed} form\u{00e1}t \u{010d}\u{00ed}sla bankovn\u{00ed}ho \u{00fa}\u{010d}tu. V \u{010c}esku za\u{010d}\u{00ed}n\u{00e1} \"CZ\" a m\u{00e1} celkem 24 znak\u{016f} (nap\u{0159}. CZ65 0800 0000 1920 0014 5399).\n\nIBAN se pou\u{017e}\u{00ed}v\u{00e1} pro zahrani\u{010d}n\u{00ed} platby, ale st\u{00e1}le \u{010d}ast\u{011b}ji i pro tuzemsk\u{00e9}. Na faktu\u{0159}e ho uv\u{00e1}d\u{011b}jte, pokud m\u{00e1}te zahrani\u{010d}n\u{00ed} odb\u{011b}ratele nebo pokud chcete u\u{017e}ivateli usnadnit platbu QR k\u{00f3}dem.".into(),
            legal: "IBAN je standardizov\u{00e1}n normou ISO 13616. V \u{010c}R je povinn\u{00fd} pro p\u{0159}eshrani\u{010d}n\u{00ed} platby v r\u{00e1}mci EU/EHP dle na\u{0159}\u{00ed}zen\u{00ed} EP a Rady (EU) \u{010d}. 260/2012 (SEPA na\u{0159}\u{00ed}zen\u{00ed}).\n\nPro tuzemsk\u{00e9} platby IBAN povinn\u{00fd} nen\u{00ed}, ale banky ho podporuj\u{00ed} a je sou\u{010d}\u{00e1}st\u{00ed} QR platebn\u{00ed}ho form\u{00e1}tu CBA. \u{010c}esk\u{00fd} IBAN m\u{00e1} form\u{00e1}t: CZ + 2 kontroln\u{00ed} \u{010d}\u{00ed}slice + 4 \u{010d}\u{00ed}slice k\u{00f3}d banky + 16 \u{010d}\u{00ed}slic \u{010d}\u{00ed}slo \u{00fa}\u{010d}tu.",
        },
        HelpTopicId::SwiftBic => HelpTopic {
            title: "SWIFT/BIC k\u{00f3}d",
            simple: "SWIFT k\u{00f3}d (tak\u{00e9} BIC) identifikuje banku p\u{0159}i mezin\u{00e1}rodn\u{00ed}ch platb\u{00e1}ch. Je to 8 nebo 11 znak\u{016f} dlouh\u{00fd} k\u{00f3}d (nap\u{0159}. KOMBCZPP pro Komer\u{010d}n\u{00ed} banku).\n\nUv\u{00e1}d\u{011b}jte ho na faktur\u{00e1}ch pro zahrani\u{010d}n\u{00ed} odb\u{011b}ratele -- bez SWIFT k\u{00f3}du nem\u{016f}\u{017e}e platba ze zahrani\u{010d}\u{00ed} doj\u{00ed}t na spr\u{00e1}vnou banku.".into(),
            legal: "SWIFT (Society for Worldwide Interbank Financial Telecommunication) k\u{00f3}d, form\u{00e1}ln\u{011b} BIC (Bank Identifier Code), je standardizov\u{00e1}n normou ISO 9362.\n\nPro platby v r\u{00e1}mci SEPA (Single Euro Payments Area) nen\u{00ed} BIC povinn\u{00fd} od 1. 2. 2016 dle na\u{0159}\u{00ed}zen\u{00ed} (EU) \u{010d}. 260/2012. Pro platby mimo SEPA je BIC st\u{00e1}le nutn\u{00fd} pro spr\u{00e1}vn\u{00e9} sm\u{011b}rov\u{00e1}n\u{00ed} platby.",
        },
        HelpTopicId::PlatceDph => HelpTopic {
            title: "Pl\u{00e1}tce DPH",
            simple: "Pl\u{00e1}tce DPH je podnikatel registrovan\u{00fd} k dani z p\u{0159}idan\u{00e9} hodnoty. Mus\u{00ed} k cen\u{00e1}m sv\u{00fd}ch slu\u{017e}eb a zbo\u{017e}\u{00ed} p\u{0159}i\u{010d}\u{00ed}t\u{00e1}vat DPH a odv\u{00e1}d\u{011b}t ho st\u{00e1}tu. Na druhou stranu si m\u{016f}\u{017e}e odpo\u{010d}\u{00ed}st DPH z n\u{00e1}kup\u{016f} souvisej\u{00ed}c\u{00ed}ch s podnik\u{00e1}n\u{00ed}m.\n\nPovinn\u{011b} se pl\u{00e1}tcem DPH st\u{00e1}v\u{00e1}te, kdy\u{017e} v\u{00e1}\u{0161} obrat za 12 po sob\u{011b} jdouc\u{00ed}ch m\u{011b}s\u{00ed}c\u{016f} p\u{0159}ekro\u{010d}\u{00ed} 2 miliony K\u{010d}. M\u{016f}\u{017e}e se st\u{00e1}t i dobrovoln\u{011b}.".into(),
            legal: "Registrace pl\u{00e1}tce DPH je upravena v \u{00a7} 6-6f z\u{00e1}kona \u{010d}. 235/2004 Sb. Povinnou registraci vyvol\u{00e1} p\u{0159}ekro\u{010d}en\u{00ed} obratu 2 000 000 K\u{010d} za 12 po sob\u{011b} jdouc\u{00ed}ch kalend\u{00e1}\u{0159}n\u{00ed}ch m\u{011b}s\u{00ed}c\u{016f} (\u{00a7} 6 odst. 1) -- platnost od 1. 1. 2025.\n\nPl\u{00e1}tce je povinen pod\u{00e1}vat da\u{0148}ov\u{00e9} p\u{0159}izn\u{00e1}n\u{00ed} (\u{00a7} 101), kontroln\u{00ed} hl\u{00e1}\u{0161}en\u{00ed} (\u{00a7} 101c) a v n\u{011b}kter\u{00fd}ch p\u{0159}\u{00ed}padech souhrnn\u{00e9} hl\u{00e1}\u{0161}en\u{00ed} (\u{00a7} 102). Da\u{0148} se odv\u{00e1}d\u{00ed} m\u{011b}s\u{00ed}\u{010d}n\u{011b} nebo \u{010d}tvrtletn\u{011b} dle \u{00a7} 99-99a.",
        },
        HelpTopicId::PriznaniDph => HelpTopic {
            title: "P\u{0159}izn\u{00e1}n\u{00ed} k DPH",
            simple: "P\u{0159}izn\u{00e1}n\u{00ed} k DPH je formul\u{00e1}\u{0159}, kter\u{00fd} pl\u{00e1}tce DPH odevzd\u{00e1}v\u{00e1} finan\u{010d}n\u{00ed}mu \u{00fa}\u{0159}adu. Obsahuje p\u{0159}ehled va\u{0161}\u{00ed} dan\u{011b} na v\u{00fd}stupu (DPH z va\u{0161}ich faktur) a dan\u{011b} na vstupu (DPH z va\u{0161}ich n\u{00e1}kup\u{016f}). Rozd\u{00ed}l bu\u{010f} zaplat\u{00ed}te st\u{00e1}tu, nebo v\u{00e1}m st\u{00e1}t vr\u{00e1}t\u{00ed}.\n\nPod\u{00e1}v\u{00e1} se m\u{011b}s\u{00ed}\u{010d}n\u{011b} nebo \u{010d}tvrtletn\u{011b}, v\u{017e}dy do 25. dne n\u{00e1}sleduj\u{00ed}c\u{00ed}ho m\u{011b}s\u{00ed}ce.".into(),
            legal: "Da\u{0148}ov\u{00e9} p\u{0159}izn\u{00e1}n\u{00ed} k DPH upravuje \u{00a7} 101 z\u{00e1}kona \u{010d}. 235/2004 Sb. Pl\u{00e1}tce je povinen podat p\u{0159}izn\u{00e1}n\u{00ed} do 25 dn\u{016f} po skon\u{010d}en\u{00ed} zda\u{0148}ovac\u{00ed}ho obdob\u{00ed} (\u{00a7} 101 odst. 1).\n\nZda\u{0148}ovac\u{00ed} obdob\u{00ed} je kalend\u{00e1}\u{0159}n\u{00ed} m\u{011b}s\u{00ed}c nebo \u{010d}tvrtlet\u{00ed} (\u{00a7} 99-99a). P\u{0159}izn\u{00e1}n\u{00ed} se pod\u{00e1}v\u{00e1} elektronicky ve form\u{00e1}tu XML na port\u{00e1}l finan\u{010d}n\u{00ed} spr\u{00e1}vy (EPO). Vzor formul\u{00e1}\u{0159}e stanov\u{00ed} Ministerstvo financ\u{00ed} vyhl\u{00e1}\u{0161}kou.",
        },
        HelpTopicId::KontrolniHlaseni => HelpTopic {
            title: "Kontroln\u{00ed} hl\u{00e1}\u{0161}en\u{00ed}",
            simple: "Kontroln\u{00ed} hl\u{00e1}\u{0161}en\u{00ed} je m\u{011b}s\u{00ed}\u{010d}n\u{00ed} report pro finan\u{010d}n\u{00ed} \u{00fa}\u{0159}ad, kter\u{00fd} obsahuje rozpis v\u{0161}ech va\u{0161}ich faktur (vydan\u{00fd}ch i p\u{0159}ijat\u{00fd}ch) s DPH. Slou\u{017e}\u{00ed} st\u{00e1}tu ke k\u{0159}\u{00ed}\u{017e}ov\u{00e9} kontrole -- ov\u{011b}\u{0159}uje, \u{017e}e DPH, kter\u{00e9} vy \u{00fa}\u{010d}tujete na v\u{00fd}stupu, si v\u{00e1}\u{0161} odb\u{011b}ratel uplatnil na vstupu, a naopak.\n\nPod\u{00e1}v\u{00e1} se v\u{017e}dy do 25. dne n\u{00e1}sleduj\u{00ed}c\u{00ed}ho m\u{011b}s\u{00ed}ce. Fyzick\u{00e9} osoby mohou pod\u{00e1}vat \u{010d}tvrtletn\u{011b}.".into(),
            legal: "Kontroln\u{00ed} hl\u{00e1}\u{0161}en\u{00ed} je upraveno v \u{00a7} 101c-101i z\u{00e1}kona \u{010d}. 235/2004 Sb. Pod\u{00e1}v\u{00e1} se elektronicky ve form\u{00e1}tu XML.\n\nLh\u{016f}ty: pr\u{00e1}vnick\u{00e9} osoby m\u{011b}s\u{00ed}\u{010d}n\u{011b}, fyzick\u{00e9} osoby ve lh\u{016f}t\u{011b} pro pod\u{00e1}n\u{00ed} da\u{0148}ov\u{00e9}ho p\u{0159}izn\u{00e1}n\u{00ed} (\u{00a7} 101e). Za nepod\u{00e1}n\u{00ed} hroz\u{00ed} pokuta 10 000-50 000 K\u{010d} (\u{00a7} 101h). Za nepod\u{00e1}n\u{00ed} na v\u{00fd}zvu a\u{017e} 500 000 K\u{010d}.\n\nObsahuje \u{00fa}daje o p\u{0159}ijat\u{00fd}ch a uskute\u{010d}n\u{011b}n\u{00fd}ch pln\u{011b}n\u{00ed}ch nad 10 000 K\u{010d} v\u{010d}etn\u{011b} DPH s identifikac\u{00ed} obchodn\u{00ed}ho partnera (DI\u{010c}).",
        },
        HelpTopicId::SouhrnneHlaseni => HelpTopic {
            title: "Souhrnn\u{00e9} hl\u{00e1}\u{0161}en\u{00ed}",
            simple: "Souhrnn\u{00e9} hl\u{00e1}\u{0161}en\u{00ed} pod\u{00e1}v\u{00e1}te, pokud dod\u{00e1}v\u{00e1}te zbo\u{017e}\u{00ed} nebo slu\u{017e}by do jin\u{00fd}ch zem\u{00ed} EU pl\u{00e1}tc\u{016f}m DPH. Hl\u{00e1}\u{0161}en\u{00ed} informuje finan\u{010d}n\u{00ed} \u{00fa}\u{0159}ad o t\u{011b}chto dod\u{00e1}vk\u{00e1}ch.\n\nPokud obchodujete pouze v \u{010c}esku, souhrnn\u{00e9} hl\u{00e1}\u{0161}en\u{00ed} v\u{00e1}s nezaj\u{00ed}m\u{00e1}.".into(),
            legal: "Souhrnn\u{00e9} hl\u{00e1}\u{0161}en\u{00ed} upravuje \u{00a7} 102 z\u{00e1}kona \u{010d}. 235/2004 Sb. Pod\u{00e1}v\u{00e1} se za ka\u{017e}d\u{00fd} kalend\u{00e1}\u{0159}n\u{00ed} m\u{011b}s\u{00ed}c (p\u{0159}i dod\u{00e1}n\u{00ed} zbo\u{017e}\u{00ed}) nebo \u{010d}tvrtlet\u{00ed} (p\u{0159}i poskytov\u{00e1}n\u{00ed} slu\u{017e}eb) do 25 dn\u{016f} po skon\u{010d}en\u{00ed} obdob\u{00ed}.\n\nT\u{00fd}k\u{00e1} se dod\u{00e1}n\u{00ed} zbo\u{017e}\u{00ed} do jin\u{00e9}ho \u{010d}lensk\u{00e9}ho st\u{00e1}tu osob\u{011b} registrovan\u{00e9} k DPH (\u{00a7} 102 odst. 1 p\u{00ed}sm. a), poskytov\u{00e1}n\u{00ed} slu\u{017e}eb s m\u{00ed}stem pln\u{011b}n\u{00ed} v jin\u{00e9}m \u{010d}lensk\u{00e9}m st\u{00e1}t\u{011b} (\u{00a7} 102 odst. 1 p\u{00ed}sm. d) a p\u{0159}em\u{00ed}st\u{011b}n\u{00ed} obchodn\u{00ed}ho majetku (\u{00a7} 102 odst. 1 p\u{00ed}sm. b).",
        },
        HelpTopicId::TypPodani => HelpTopic {
            title: "Typ pod\u{00e1}n\u{00ed}",
            simple: "Typ pod\u{00e1}n\u{00ed} ur\u{010d}uje, zda se jedn\u{00e1} o \u{0159}\u{00e1}dn\u{00e9}, opravn\u{00e9} nebo dodate\u{010d}n\u{00e9} pod\u{00e1}n\u{00ed}:\n\n- \u{0158}\u{00e1}dn\u{00e9} -- prvn\u{00ed} pod\u{00e1}n\u{00ed} za dan\u{00e9} obdob\u{00ed}\n- Opravn\u{00e9} -- oprava pod\u{00e1}n\u{00ed} p\u{0159}ed uplynut\u{00ed}m lh\u{016f}ty (nahrad\u{00ed} p\u{016f}vodn\u{00ed})\n- Dodate\u{010d}n\u{00e9} -- oprava po uplynut\u{00ed} lh\u{016f}ty (pod\u{00e1}v\u{00e1} se nav\u{00ed}c k \u{0159}\u{00e1}dn\u{00e9}mu)".into(),
            legal: "Typy pod\u{00e1}n\u{00ed} definuje z\u{00e1}kon \u{010d}. 280/2009 Sb. (da\u{0148}ov\u{00fd} \u{0159}\u{00e1}d):\n\n- \u{0158}\u{00e1}dn\u{00e9} pod\u{00e1}n\u{00ed} (\u{00a7} 135) -- standardn\u{00ed} pod\u{00e1}n\u{00ed} v z\u{00e1}konn\u{00e9}m term\u{00ed}nu\n- Opravn\u{00e9} pod\u{00e1}n\u{00ed} (\u{00a7} 138) -- nahrazuje p\u{016f}vodn\u{00ed} pod\u{00e1}n\u{00ed} p\u{0159}ed uplynut\u{00ed}m lh\u{016f}ty, posledn\u{00ed} podan\u{00e9} plat\u{00ed}\n- Dodate\u{010d}n\u{00e9} pod\u{00e1}n\u{00ed} (\u{00a7} 141) -- pod\u{00e1}v\u{00e1} se po uplynut\u{00ed} lh\u{016f}ty pro \u{0159}\u{00e1}dn\u{00e9} pod\u{00e1}n\u{00ed}, pokud podnikatel zjist\u{00ed} chybu. Lh\u{016f}ta pro pod\u{00e1}n\u{00ed}: do konce m\u{011b}s\u{00ed}ce n\u{00e1}sleduj\u{00ed}c\u{00ed}ho po zji\u{0161}t\u{011b}n\u{00ed} chyby",
        },
        HelpTopicId::CiselneRady => HelpTopic {
            title: "\u{010c}\u{00ed}seln\u{00e9} \u{0159}ady",
            simple: "\u{010c}\u{00ed}seln\u{00e9} \u{0159}ady zaji\u{0161}\u{0165}uj\u{00ed} automatick\u{00e9} \u{010d}\u{00ed}slov\u{00e1}n\u{00ed} va\u{0161}ich faktur. M\u{00ed}sto ru\u{010d}n\u{00ed}ho zad\u{00e1}v\u{00e1}n\u{00ed} \u{010d}\u{00ed}sel syst\u{00e9}m s\u{00e1}m p\u{0159}i\u{0159}ad\u{00ed} dal\u{0161}\u{00ed} \u{010d}\u{00ed}slo v po\u{0159}ad\u{00ed}.\n\nM\u{016f}\u{017e}ete m\u{00ed}t v\u{00ed}ce \u{010d}\u{00ed}seln\u{00fd}ch \u{0159}ad -- nap\u{0159}. jednu pro tuzemsk\u{00e9} faktury (FV-2024-001) a jinou pro zahrani\u{010d}n\u{00ed} (ZF-2024-001).".into(),
            legal: "Povinnost \u{010d}\u{00ed}seln\u{00fd}ch \u{0159}ad vypl\u{00fd}v\u{00e1} z \u{00a7} 29 odst. 1 p\u{00ed}sm. b) z\u{00e1}kona \u{010d}. 235/2004 Sb. -- da\u{0148}ov\u{00fd} doklad mus\u{00ed} obsahovat po\u{0159}adov\u{00e9} \u{010d}\u{00ed}slo p\u{0159}i\u{0159}azen\u{00e9} v r\u{00e1}mci jedn\u{00e9} \u{010d}i v\u{00ed}ce \u{010d}\u{00ed}seln\u{00fd}ch \u{0159}ad.\n\n\u{010c}\u{00ed}seln\u{00e1} \u{0159}ada mus\u{00ed} zaru\u{010d}ovat jednozna\u{010d}nost dokladu. Podnikatel je povinen v\u{00e9}st evidenci vydan\u{00fd}ch doklad\u{016f} a jejich \u{010d}\u{00ed}seln\u{00fd}ch \u{0159}ad pro \u{00fa}\u{010d}ely p\u{0159}\u{00ed}padn\u{00e9} kontroly finan\u{010d}n\u{00ed}m \u{00fa}\u{0159}adem.",
        },
        HelpTopicId::PrefixFormat => HelpTopic {
            title: "Prefix a form\u{00e1}t \u{010d}\u{00ed}seln\u{00e9} \u{0159}ady",
            simple: "Prefix je text p\u{0159}ed \u{010d}\u{00ed}slem faktury (nap\u{0159}. \"FV\" pro fakturu vydanou). Form\u{00e1}t ur\u{010d}uje, jak bude \u{010d}\u{00ed}slo vypadat -- nap\u{0159}. \"{prefix}{year}-{number:4}\" vytvo\u{0159}\u{00ed} \u{010d}\u{00ed}sla jako FV2024-0001, FV2024-0002 atd.\n\n\u{010c}\u{00ed}slov\u{00e1}n\u{00ed} se resetuje na za\u{010d}\u{00e1}tku ka\u{017e}d\u{00e9}ho roku, tak\u{017e}e prvn\u{00ed} faktura nov\u{00e9}ho roku bude v\u{017e}dy 0001.".into(),
            legal: "Form\u{00e1}t \u{010d}\u{00ed}seln\u{00e9} \u{0159}ady nen\u{00ed} z\u{00e1}konem p\u{0159}edeps\u{00e1}n. Z\u{00e1}kon \u{010d}. 235/2004 Sb. v \u{00a7} 29 vy\u{017e}aduje pouze to, aby po\u{0159}adov\u{00e9} \u{010d}\u{00ed}slo bylo jednozna\u{010d}n\u{00e9} v r\u{00e1}mci \u{010d}\u{00ed}seln\u{00e9} \u{0159}ady.\n\nDoporu\u{010d}uje se v\u{010d}etn\u{011b} roku (nap\u{0159}. 2024-001) pro snaz\u{0161}\u{00ed} orientaci a pr\u{016f}kaznost p\u{0159}i da\u{0148}ov\u{00e9} kontrole. Prefix pom\u{00e1}h\u{00e1} rozli\u{0161}it typ dokladu (faktury vydan\u{00e9}, p\u{0159}ijat\u{00e9}, dobropisy atd.).",
        },
        HelpTopicId::PrijmyNaklady => HelpTopic {
            title: "P\u{0159}\u{00ed}jmy a n\u{00e1}klady",
            simple: "P\u{0159}\u{00ed}jmy jsou pen\u{00ed}ze, kter\u{00e9} v\u{00e1}m z\u{00e1}kazn\u{00ed}ci zaplatili za va\u{0161}e slu\u{017e}by nebo zbo\u{017e}\u{00ed}. N\u{00e1}klady jsou pen\u{00ed}ze, kter\u{00e9} jste utratili v souvislosti s podnik\u{00e1}n\u{00ed}m.\n\nRozd\u{00ed}l mezi p\u{0159}\u{00ed}jmy a n\u{00e1}klady je z\u{00e1}kladem dan\u{011b} -- \u{010d}\u{00ed}m v\u{00ed}ce n\u{00e1}klad\u{016f} (da\u{0148}ov\u{011b} uznateln\u{00fd}ch) m\u{00e1}te, t\u{00ed}m m\u{00e9}n\u{011b} dan\u{011b} zaplat\u{00ed}te.".into(),
            legal: "P\u{0159}\u{00ed}jmy z podnik\u{00e1}n\u{00ed} OSV\u{010c} jsou upraveny v \u{00a7} 7 z\u{00e1}kona \u{010d}. 586/1992 Sb. o dan\u{00ed}ch z p\u{0159}\u{00ed}jm\u{016f}. Z\u{00e1}klad dan\u{011b} se stanov\u{00ed} jako rozd\u{00ed}l mezi p\u{0159}\u{00ed}jmy a v\u{00fd}daji (\u{00a7} 23).\n\nAlternativn\u{011b} m\u{016f}\u{017e}e OSV\u{010c} uplatnit pau\u{0161}\u{00e1}ln\u{00ed} v\u{00fd}daje (\u{00a7} 7 odst. 7): 80 % u \u{0159}emesln\u{00fd}ch \u{017e}ivnost\u{00ed}, 60 % u ostatn\u{00ed}ch \u{017e}ivnost\u{00ed}, 40 % u p\u{0159}\u{00ed}jm\u{016f} z jin\u{00e9}ho podnik\u{00e1}n\u{00ed}. Pau\u{0161}\u{00e1}ln\u{00ed} v\u{00fd}daje jsou omezeny \u{010d}\u{00e1}stkou 1 600 000 / 1 200 000 / 800 000 K\u{010d}.",
        },
        HelpTopicId::NeuhrazeneFaktury => HelpTopic {
            title: "Neuhrazen\u{00e9} faktury",
            simple: "Neuhrazen\u{00e9} faktury jsou faktury, kter\u{00e9} jste vystavili, ale z\u{00e1}kazn\u{00ed}k je je\u{0161}t\u{011b} nezaplatil. Mohou b\u{00fd}t p\u{0159}ed splatnost\u{00ed} (z\u{00e1}kazn\u{00ed}k m\u{00e1} je\u{0161}t\u{011b} \u{010d}as) nebo po splatnosti (z\u{00e1}kazn\u{00ed}k je v prodlen\u{00ed}).\n\nJe d\u{016f}le\u{017e}it\u{00e9} sledovat neuhrazen\u{00e9} faktury a v\u{010d}as upom\u{00ed}nat dlu\u{017e}n\u{00ed}ky. Po splatnosti maj\u{00ed} \u{00fa}roky z prodlen\u{00ed}.".into(),
            legal: "Neuhrazen\u{00e9} pohled\u{00e1}vky po splatnosti lze da\u{0148}ov\u{011b} odepsat dle \u{00a7} 24 odst. 2 p\u{00ed}sm. y) z\u{00e1}kona \u{010d}. 586/1992 Sb. u pohled\u{00e1}vek za dlu\u{017e}n\u{00ed}kem v insolven\u{010d}n\u{00ed}m \u{0159}\u{00ed}zen\u{00ed}.\n\nOpravn\u{00e9} polo\u{017e}ky k pohled\u{00e1}vk\u{00e1}m upravuje z\u{00e1}kon \u{010d}. 593/1992 Sb. o rezerv\u{00e1}ch: po 18 m\u{011b}s\u{00ed}c\u{00ed}ch po splatnosti a\u{017e} 50 %, po 30 m\u{011b}s\u{00ed}c\u{00ed}ch a\u{017e} 100 % (\u{00a7} 8a). \u{00da}roky z prodlen\u{00ed} se \u{0159}\u{00ed}d\u{00ed} \u{00a7} 1970 ob\u{010d}ansk\u{00e9}ho z\u{00e1}kon\u{00ed}ku -- repo sazba \u{010c}NB + 8 p.b.",
        },
        HelpTopicId::FakturyPoSplatnosti => HelpTopic {
            title: "Faktury po splatnosti",
            simple: "Faktura je po splatnosti, kdy\u{017e} z\u{00e1}kazn\u{00ed}k nezaplatil do data splatnosti. Od tohoto okam\u{017e}iku je v prodlen\u{00ed} a vy m\u{016f}\u{017e}ete uplatnit \u{00fa}roky z prodlen\u{00ed}.\n\nDoporu\u{010d}en\u{00fd} postup: po 7 dnech prvn\u{00ed} upom\u{00ed}nka, po 14 dnech druh\u{00e1} upom\u{00ed}nka, po 30 dnech p\u{0159}edtoudn\u{00ed} upom\u{00ed}nka s v\u{00fd}hru\u{017e}kou pr\u{00e1}vn\u{00ed}mi kroky.".into(),
            legal: "Prodlen\u{00ed} dlu\u{017e}n\u{00ed}ka upravuje \u{00a7} 1968-1975 z\u{00e1}kona \u{010d}. 89/2012 Sb. (ob\u{010d}ansk\u{00fd} z\u{00e1}kon\u{00ed}k). Dlu\u{017e}n\u{00ed}k, kter\u{00fd} sv\u{016f}j dluh \u{0159}\u{00e1}dn\u{011b} a v\u{010d}as nepln\u{00ed}, je v prodlen\u{00ed} (\u{00a7} 1968).\n\nV\u{011b}\u{0159}itel m\u{00e1} pr\u{00e1}vo na \u{00fa}roky z prodlen\u{00ed} (\u{00a7} 1970) ve v\u{00fd}\u{0161}i repo sazby \u{010c}NB + 8 procentn\u{00ed}ch bod\u{016f}. U obchodn\u{00ed}ch vztah\u{016f} m\u{00e1} v\u{011b}\u{0159}itel tak\u{00e9} pr\u{00e1}vo na minim\u{00e1}ln\u{00ed} pau\u{0161}\u{00e1}l 1 200 K\u{010d} za n\u{00e1}klady spojen\u{00e9} s uplatn\u{011b}n\u{00ed}m pohled\u{00e1}vky (na\u{0159}\u{00ed}zen\u{00ed} vl\u{00e1}dy \u{010d}. 351/2013 Sb.).",
        },
        HelpTopicId::FrekvenceOpakovani => HelpTopic {
            title: "Frekvence opakov\u{00e1}n\u{00ed}",
            simple: "Frekvence ur\u{010d}uje, jak \u{010d}asto se opakuj\u{00ed}c\u{00ed} faktura automaticky vytvo\u{0159}\u{00ed}. Nap\u{0159}. m\u{011b}s\u{00ed}\u{010d}n\u{00ed} frekvence znamen\u{00e1}, \u{017e}e se faktura vytvo\u{0159}\u{00ed} jednou m\u{011b}s\u{00ed}\u{010d}n\u{011b}.\n\nB\u{011b}\u{017e}n\u{00e9} frekvence: m\u{011b}s\u{00ed}\u{010d}n\u{00ed} (nap\u{0159}. pau\u{0161}\u{00e1}ln\u{00ed} slu\u{017e}by, n\u{00e1}jem), \u{010d}tvrtletn\u{00ed} (nap\u{0159}. pravideln\u{00e9} konzultace), ro\u{010d}n\u{00ed} (nap\u{0159}. licence, p\u{0159}edplatn\u{00e9}).".into(),
            legal: "Opakuj\u{00ed}c\u{00ed} se pln\u{011b}n\u{00ed} (trval\u{00e9} pln\u{011b}n\u{00ed}) je upraveno v \u{00a7} 21 odst. 8 z\u{00e1}kona \u{010d}. 235/2004 Sb. U opakovan\u{00e9}ho pln\u{011b}n\u{00ed} se DUZP stanov\u{00ed} nejpozd\u{011b}ji posledn\u{00ed}m dnem zda\u{0148}ovac\u{00ed}ho obdob\u{00ed}.\n\nSmlouvy na opakovan\u{00e9} pln\u{011b}n\u{00ed} (nap\u{0159}. n\u{00e1}jem, servisn\u{00ed} smlouvy) se \u{0159}\u{00ed}d\u{00ed} ustanoven\u{00ed}mi o z\u{00e1}vazkov\u{00e9}m pr\u{00e1}vu v ob\u{010d}ansk\u{00e9}m z\u{00e1}kon\u{00ed}ku (\u{00a7} 1724 a n\u{00e1}sl. z\u{00e1}kona \u{010d}. 89/2012 Sb.).",
        },
        HelpTopicId::VystupniDph => HelpTopic {
            title: "V\u{00fd}stupn\u{00ed} DPH",
            simple: "V\u{00fd}stupn\u{00ed} DPH je da\u{0148}, kterou \u{00fa}\u{010d}tujete sv\u{00fd}m z\u{00e1}kazn\u{00ed}k\u{016f}m na faktur\u{00e1}ch. Kdy\u{017e} vystav\u{00ed}te fakturu s DPH, tuto da\u{0148} mus\u{00ed}te odv\u{00e9}st st\u{00e1}tu.\n\nNap\u{0159}. fakturujete slu\u{017e}bu za 10 000 K\u{010d} + 21 % DPH = 12 100 K\u{010d}. T\u{011b}ch 2 100 K\u{010d} je v\u{00fd}stupn\u{00ed} DPH, kter\u{00e9} odvedete finan\u{010d}n\u{00ed}mu \u{00fa}\u{0159}adu.".into(),
            legal: "V\u{00fd}stupn\u{00ed} DPH (da\u{0148} na v\u{00fd}stupu) je definov\u{00e1}no v \u{00a7} 4 odst. 1 p\u{00ed}sm. c) z\u{00e1}kona \u{010d}. 235/2004 Sb. o DPH. Pl\u{00e1}tce je povinen p\u{0159}iznat da\u{0148} na v\u{00fd}stupu ke dni uskute\u{010d}n\u{011b}n\u{00ed} zdaniteln\u{00e9}ho pln\u{011b}n\u{00ed} (\u{00a7} 20a) nebo ke dni p\u{0159}ijet\u{00ed} \u{00fa}hrady, pokud nastala d\u{0159}\u{00ed}ve (\u{00a7} 21).\n\nDa\u{0148} na v\u{00fd}stupu se uv\u{00e1}d\u{00ed} v da\u{0148}ov\u{00e9}m p\u{0159}izn\u{00e1}n\u{00ed} v \u{0159}\u{00e1}dc\u{00ed}ch 1-13 formul\u{00e1}\u{0159}e.",
        },
        HelpTopicId::VstupniDph => HelpTopic {
            title: "Vstupn\u{00ed} DPH",
            simple: "Vstupn\u{00ed} DPH je da\u{0148}, kterou jste zaplatili p\u{0159}i sv\u{00fd}ch n\u{00e1}kupech. Tuto da\u{0148} si m\u{016f}\u{017e}ete ode\u{010d}\u{00ed}st od v\u{00fd}stupn\u{00ed}ho DPH -- t\u{00ed}m sn\u{00ed}\u{017e}\u{00ed}te \u{010d}\u{00e1}stku, kterou odvedete st\u{00e1}tu.\n\nNap\u{0159}. koup\u{00ed}te notebook za 24 200 K\u{010d} (20 000 + 4 200 DPH). T\u{011b}ch 4 200 K\u{010d} je vstupn\u{00ed} DPH, kter\u{00e9} si ode\u{010d}tete.".into(),
            legal: "N\u{00e1}rok na odpo\u{010d}et dan\u{011b} na vstupu upravuj\u{00ed} \u{00a7} 72-73 z\u{00e1}kona \u{010d}. 235/2004 Sb. Pl\u{00e1}tce m\u{00e1} n\u{00e1}rok na odpo\u{010d}et dan\u{011b} u p\u{0159}ijat\u{00fd}ch zdaniteln\u{00fd}ch pln\u{011b}n\u{00ed}, kter\u{00e1} pou\u{017e}ije pro uskute\u{010d}n\u{011b}n\u{00ed} sv\u{00e9} ekonomick\u{00e9} \u{010d}innosti (\u{00a7} 72 odst. 1).\n\nPodm\u{00ed}nkou odpo\u{010d}tu je dr\u{017e}en\u{00ed} da\u{0148}ov\u{00e9}ho dokladu (\u{00a7} 73 odst. 1). N\u{00e1}rok na odpo\u{010d}et lze uplatnit nejd\u{0159}\u{00ed}ve za zda\u{0148}ovac\u{00ed} obdob\u{00ed}, ve kter\u{00e9}m jsou spln\u{011b}ny podm\u{00ed}nky (\u{00a7} 73 odst. 3).",
        },
        HelpTopicId::PreneseniDanovePovinnosti => HelpTopic {
            title: "P\u{0159}enesen\u{00ed} da\u{0148}ov\u{00e9} povinnosti",
            simple: "P\u{0159}enesen\u{00ed} da\u{0148}ov\u{00e9} povinnosti (reverse charge) znamen\u{00e1}, \u{017e}e DPH neplat\u{00ed} dodavatel, ale odb\u{011b}ratel. Dodavatel vystav\u{00ed} fakturu bez DPH a odb\u{011b}ratel si DPH s\u{00e1}m vypo\u{010d}\u{00ed}t\u{00e1} a p\u{0159}izn\u{00e1}.\n\nPou\u{017e}\u{00ed}v\u{00e1} se nap\u{0159}. u stavebn\u{00ed}ch prac\u{00ed}, dod\u{00e1}n\u{00ed} \u{0161}rotu a odpadu, nebo u obchod\u{016f} mezi firmami v r\u{00e1}mci EU.".into(),
            legal: "P\u{0159}enesen\u{00ed} da\u{0148}ov\u{00e9} povinnosti (re\u{017e}im reverse charge) upravuje \u{00a7} 92a z\u{00e1}kona \u{010d}. 235/2004 Sb. U tuzemsk\u{00fd}ch pln\u{011b}n\u{00ed} se t\u{00fd}k\u{00e1} zbo\u{017e}\u{00ed} a slu\u{017e}eb uveden\u{00fd}ch v p\u{0159}\u{00ed}loze \u{010d}. 6 z\u{00e1}kona (stavebn\u{00ed} pr\u{00e1}ce, \u{0161}rot, odpady aj.).\n\nP\u{0159}i p\u{0159}enesen\u{00ed} da\u{0148}ov\u{00e9} povinnosti je odb\u{011b}ratel povinen da\u{0148} p\u{0159}iznat a m\u{00e1} n\u{00e1}rok na odpo\u{010d}et (\u{00a7} 92a odst. 1). Dodavatel uvede pln\u{011b}n\u{00ed} v \u{0159}\u{00e1}dku 25 da\u{0148}ov\u{00e9}ho p\u{0159}izn\u{00e1}n\u{00ed}.",
        },
        HelpTopicId::NadmernyOdpocet => HelpTopic {
            title: "Nadm\u{011b}rn\u{00fd} odpo\u{010d}et / Da\u{0148}ov\u{00e1} povinnost",
            simple: "V\u{00fd}sledek DPH p\u{0159}izn\u{00e1}n\u{00ed} je bu\u{010f} da\u{0148}ov\u{00e1} povinnost, nebo nadm\u{011b}rn\u{00fd} odpo\u{010d}et:\n\n- Da\u{0148}ov\u{00e1} povinnost: v\u{00fd}stupn\u{00ed} DPH > vstupn\u{00ed} DPH -- rozd\u{00ed}l zaplat\u{00ed}te st\u{00e1}tu\n- Nadm\u{011b}rn\u{00fd} odpo\u{010d}et: vstupn\u{00ed} DPH > v\u{00fd}stupn\u{00ed} DPH -- st\u{00e1}t v\u{00e1}m vr\u{00e1}t\u{00ed} rozd\u{00ed}l\n\nNadm\u{011b}rn\u{00fd} odpo\u{010d}et vznik\u{00e1} nap\u{0159}. p\u{0159}i velk\u{00fd}ch investic\u{00ed}ch (n\u{00e1}kup stroje, rekonstrukce).".into(),
            legal: "Nadm\u{011b}rn\u{00fd} odpo\u{010d}et je definov\u{00e1}n v \u{00a7} 4 odst. 1 p\u{00ed}sm. d) z\u{00e1}kona \u{010d}. 235/2004 Sb. Vznikne-li nadm\u{011b}rn\u{00fd} odpo\u{010d}et, vr\u{00e1}t\u{00ed} ho spr\u{00e1}vce dan\u{011b} pl\u{00e1}tci do 30 dn\u{016f} od vym\u{011b}\u{0159}en\u{00ed} (\u{00a7} 105 odst. 1).\n\nSpr\u{00e1}vce dan\u{011b} m\u{016f}\u{017e}e p\u{0159}ed vr\u{00e1}cen\u{00ed}m zah\u{00e1}jit postup k odstran\u{011b}n\u{00ed} pochybnost\u{00ed} (\u{00a7} 89 da\u{0148}ov\u{00e9}ho \u{0159}\u{00e1}du), \u{010d}\u{00ed}m\u{017e} se lh\u{016f}ta prodlou\u{017e}\u{00ed}. Nadm\u{011b}rn\u{00fd} odpo\u{010d}et se p\u{0159}ednostn\u{011b} pou\u{017e}ije na \u{00fa}hradu p\u{0159}\u{00ed}padn\u{00fd}ch da\u{0148}ov\u{00fd}ch nedoplatk\u{016f} (\u{00a7} 105 odst. 2).",
        },
        HelpTopicId::ZakladDane => HelpTopic {
            title: "Z\u{00e1}klad dan\u{011b}",
            simple: "Z\u{00e1}klad dan\u{011b} je \u{010d}\u{00e1}stka bez DPH, ze kter\u{00e9} se DPH vypo\u{010d}\u{00ed}t\u{00e1}. Nap\u{0159}. pokud je cena slu\u{017e}by 12 100 K\u{010d} v\u{010d}etn\u{011b} 21 % DPH, z\u{00e1}klad dan\u{011b} je 10 000 K\u{010d} a DPH 2 100 K\u{010d}.\n\nV DPH p\u{0159}izn\u{00e1}n\u{00ed} se z\u{00e1}klad dan\u{011b} uv\u{00e1}d\u{00ed} ve sloupc\u{00ed}ch vedle vypo\u{010d}ten\u{00e9} dan\u{011b}.".into(),
            legal: "Z\u{00e1}klad dan\u{011b} je definov\u{00e1}n v \u{00a7} 36 z\u{00e1}kona \u{010d}. 235/2004 Sb. Z\u{00e1}kladem dan\u{011b} je v\u{0161}e, co jako \u{00fa}hradu obdr\u{017e}el nebo m\u{00e1} obdr\u{017e}et pl\u{00e1}tce za uskute\u{010d}n\u{011b}n\u{00e9} zdaniteln\u{00e9} pln\u{011b}n\u{00ed} od osoby, pro kterou pln\u{011b}n\u{00ed} uskute\u{010d}nil, nebo od t\u{0159}et\u{00ed} osoby (\u{00a7} 36 odst. 1).\n\nZ\u{00e1}klad dan\u{011b} zahrnuje i vedlej\u{0161}\u{00ed} v\u{00fd}daje (balen\u{00ed}, p\u{0159}eprava, poji\u{0161}t\u{011b}n\u{00ed}) dle \u{00a7} 36 odst. 3.",
        },
        HelpTopicId::SekceKontrolniHlaseni => HelpTopic {
            title: "Sekce kontroln\u{00ed}ho hl\u{00e1}\u{0161}en\u{00ed} (A4/A5/B2/B3)",
            simple: "Kontroln\u{00ed} hl\u{00e1}\u{0161}en\u{00ed} se d\u{011b}l\u{00ed} na sekce podle sm\u{011b}ru a velikosti pln\u{011b}n\u{00ed}:\n\n- A4: Vydan\u{00e9} faktury nad 10 000 K\u{010d} v\u{010d}etn\u{011b} DPH (s detailem o odb\u{011b}rateli)\n- A5: Vydan\u{00e9} faktury do 10 000 K\u{010d} (souhrnn\u{011b}, bez detailu)\n- B2: P\u{0159}ijat\u{00e9} faktury nad 10 000 K\u{010d} v\u{010d}etn\u{011b} DPH (s detailem o dodavateli)\n- B3: P\u{0159}ijat\u{00e9} faktury do 10 000 K\u{010d} (souhrnn\u{011b}, bez detailu)\n\nU A4 a B2 se uv\u{00e1}d\u{00ed} DI\u{010c} partnera, \u{010d}\u{00ed}slo dokladu a dal\u{0161}\u{00ed} \u{00fa}daje.".into(),
            legal: "\u{010c}len\u{011b}n\u{00ed} kontroln\u{00ed}ho hl\u{00e1}\u{0161}en\u{00ed} stanovuje \u{00a7} 101c-101d z\u{00e1}kona \u{010d}. 235/2004 Sb. a pokyn GF\u{0158}-D-57.\n\nOdd\u{00ed}l A obsahuje \u{00fa}daje o uskute\u{010d}n\u{011b}n\u{00fd}ch pln\u{011b}n\u{00ed}ch (v\u{00fd}stupy): A4 = pln\u{011b}n\u{00ed} nad 10 000 K\u{010d} s identifikac\u{00ed} odb\u{011b}ratele, A5 = ostatn\u{00ed} pln\u{011b}n\u{00ed}. Odd\u{00ed}l B obsahuje \u{00fa}daje o p\u{0159}ijat\u{00fd}ch pln\u{011b}n\u{00ed}ch (vstupy): B2 = pln\u{011b}n\u{00ed} nad 10 000 K\u{010d} s identifikac\u{00ed} dodavatele, B3 = ostatn\u{00ed} pln\u{011b}n\u{00ed}.\n\nRozhoduj\u{00ed}c\u{00ed} \u{010d}\u{00e1}stka 10 000 K\u{010d} je v\u{010d}etn\u{011b} DPH.",
        },
        HelpTopicId::Dppd => HelpTopic {
            title: "Datum poskytnut\u{00ed} da\u{0148}ov\u{00e9}ho pln\u{011b}n\u{00ed} (DPPD)",
            simple: "DPPD je datum, kter\u{00e9} se uv\u{00e1}d\u{00ed} v kontroln\u{00ed}m hl\u{00e1}\u{0161}en\u{00ed}. Odpov\u{00ed}d\u{00e1} datu uskute\u{010d}n\u{011b}n\u{00ed} pln\u{011b}n\u{00ed} (DUZP) z faktury.\n\nPozor: DPPD nen\u{00ed} datum vystaven\u{00ed} faktury ani datum splatnosti -- je to den, kdy skute\u{010d}n\u{011b} do\u{0161}lo k dod\u{00e1}n\u{00ed} zbo\u{017e}\u{00ed} nebo poskytnut\u{00ed} slu\u{017e}by.".into(),
            legal: "DPPD (datum poskytnut\u{00ed}/p\u{0159}ijet\u{00ed} pln\u{011b}n\u{00ed}) se uv\u{00e1}d\u{00ed} v kontroln\u{00ed}m hl\u{00e1}\u{0161}en\u{00ed} dle \u{00a7} 101c z\u{00e1}kona \u{010d}. 235/2004 Sb. Odpov\u{00ed}d\u{00e1} datu uskute\u{010d}n\u{011b}n\u{00ed} zdaniteln\u{00e9}ho pln\u{011b}n\u{00ed} (DUZP) dle \u{00a7} 21 t\u{00e9}ho\u{017e} z\u{00e1}kona.\n\nV odd\u{00ed}lech A4 a B2 kontroln\u{00ed}ho hl\u{00e1}\u{0161}en\u{00ed} se DPPD uv\u{00e1}d\u{00ed} u ka\u{017e}d\u{00e9}ho \u{0159}\u{00e1}dku. V odd\u{00ed}lech A5 a B3 se neuv\u{00e1}d\u{00ed} (pln\u{011b}n\u{00ed} jsou agregov\u{00e1}na).",
        },
        HelpTopicId::KodPlneni => HelpTopic {
            title: "K\u{00f3}d pln\u{011b}n\u{00ed}",
            simple: "K\u{00f3}d pln\u{011b}n\u{00ed} v souhrnn\u{00e9}m hl\u{00e1}\u{0161}en\u{00ed} ur\u{010d}uje typ obchodu s partnerem v EU:\n\n- 0: Dod\u{00e1}n\u{00ed} zbo\u{017e}\u{00ed} do jin\u{00e9} \u{010d}lensk\u{00e9} zem\u{011b}\n- 1: Poskytnut\u{00ed} slu\u{017e}by podle \u{00a7} 9 odst. 1 (m\u{00ed}sto pln\u{011b}n\u{00ed} u p\u{0159}\u{00ed}jemce)\n- 2: Obchod v r\u{00e1}mci triangulace (t\u{0159}et\u{00ed} strana)\n- 3: Poskytnut\u{00ed} slu\u{017e}by podle \u{00a7} 54 (finan\u{010d}n\u{00ed} a poji\u{0161}\u{0165}ovac\u{00ed} slu\u{017e}by)".into(),
            legal: "K\u{00f3}dy pln\u{011b}n\u{00ed} jsou definov\u{00e1}ny v \u{00a7} 102 z\u{00e1}kona \u{010d}. 235/2004 Sb. a v pokynu GF\u{0158} k vypln\u{011b}n\u{00ed} souhrnn\u{00e9}ho hl\u{00e1}\u{0161}en\u{00ed}.\n\nK\u{00f3}d 0: dod\u{00e1}n\u{00ed} zbo\u{017e}\u{00ed} osob\u{011b} registrovan\u{00e9} k DPH v jin\u{00e9}m \u{010d}lensk\u{00e9}m st\u{00e1}t\u{011b} (\u{00a7} 102 odst. 1 p\u{00ed}sm. a). K\u{00f3}d 1: poskytnut\u{00ed} slu\u{017e}by s m\u{00ed}stem pln\u{011b}n\u{00ed} dle \u{00a7} 9 odst. 1 (\u{00a7} 102 odst. 1 p\u{00ed}sm. d). K\u{00f3}d 2: dod\u{00e1}n\u{00ed} zbo\u{017e}\u{00ed} v r\u{00e1}mci zjednodu\u{0161}en\u{00e9}ho postupu p\u{0159}i t\u{0159}\u{00ed}strann\u{00e9}m obchod\u{011b} (\u{00a7} 102 odst. 1 p\u{00ed}sm. c). K\u{00f3}d 3: poskytnut\u{00ed} slu\u{017e}by dle \u{00a7} 54.",
        },
        HelpTopicId::ZdanovaciObdobi => HelpTopic {
            title: "Zda\u{0148}ovac\u{00ed} obdob\u{00ed}",
            simple: "Zda\u{0148}ovac\u{00ed} obdob\u{00ed} je \u{010d}asov\u{00fd} \u{00fa}sek, za kter\u{00fd} pod\u{00e1}v\u{00e1}te DPH p\u{0159}izn\u{00e1}n\u{00ed} a odv\u{00e1}d\u{00ed}te da\u{0148}. M\u{016f}\u{017e}e b\u{00fd}t:\n\n- M\u{011b}s\u{00ed}\u{010d}n\u{00ed}: p\u{0159}izn\u{00e1}n\u{00ed} pod\u{00e1}v\u{00e1}te ka\u{017e}d\u{00fd} m\u{011b}s\u{00ed}c (povinn\u{011b} p\u{0159}i obratu nad 10 mil. K\u{010d})\n- \u{010c}tvrtletn\u{00ed}: p\u{0159}izn\u{00e1}n\u{00ed} pod\u{00e1}v\u{00e1}te za ka\u{017e}d\u{00e9} \u{010d}tvrtlet\u{00ed} (pro men\u{0161}\u{00ed} pl\u{00e1}tce DPH)\n\nP\u{0159}izn\u{00e1}n\u{00ed} se v\u{017e}dy pod\u{00e1}v\u{00e1} do 25. dne po skon\u{010d}en\u{00ed} obdob\u{00ed}.".into(),
            legal: "Zda\u{0148}ovac\u{00ed} obdob\u{00ed} upravuj\u{00ed} \u{00a7} 99-99a z\u{00e1}kona \u{010d}. 235/2004 Sb. Z\u{00e1}kladn\u{00ed}m zda\u{0148}ovac\u{00ed}m obdob\u{00ed}m je kalend\u{00e1}\u{0159}n\u{00ed} m\u{011b}s\u{00ed}c (\u{00a7} 99). Pl\u{00e1}tce m\u{016f}\u{017e}e zvolit \u{010d}tvrtletn\u{00ed} obdob\u{00ed}, pokud jeho obrat za p\u{0159}edch\u{00e1}zej\u{00ed}c\u{00ed} kalend\u{00e1}\u{0159}n\u{00ed} rok nep\u{0159}es\u{00e1}hl 10 000 000 K\u{010d} a nen\u{00ed} nespolehliv\u{00fd}m pl\u{00e1}tcem (\u{00a7} 99a).\n\nZm\u{011b}na zda\u{0148}ovac\u{00ed}ho obdob\u{00ed} se oznamuje spr\u{00e1}vci dan\u{011b} do konce ledna p\u{0159}\u{00ed}slu\u{0161}n\u{00e9}ho roku (\u{00a7} 99a odst. 2).",
        },
        HelpTopicId::TypFaktury => HelpTopic {
            title: "Typ dokladu",
            simple: "Faktura je da\u{0148}ov\u{00fd} doklad, kter\u{00fd} vystavujete za dodan\u{00e9} zbo\u{017e}\u{00ed} nebo slu\u{017e}by. Z\u{00e1}lohov\u{00e1} faktura (proforma) je v\u{00fd}zva k platb\u{011b} -- nen\u{00ed} da\u{0148}ov\u{00fd}m dokladem a neslou\u{017e}\u{00ed} k uplatn\u{011b}n\u{00ed} DPH.\n\nPokud jste pl\u{00e1}tce DPH, po \u{00fa}hrad\u{011b} z\u{00e1}lohov\u{00e9} faktury mus\u{00ed}te vystavit \u{0159}\u{00e1}dnou fakturu (vyrovn\u{00e1}n\u{00ed} z\u{00e1}lohy).".into(),
            legal: "Da\u{0148}ov\u{00fd} doklad je definov\u{00e1}n v \u{00a7} 26 z\u{00e1}kona \u{010d}. 235/2004 Sb. o DPH. Z\u{00e1}lohov\u{00e1} faktura nen\u{00ed} da\u{0148}ov\u{00fd}m dokladem ve smyslu tohoto z\u{00e1}kona -- jedn\u{00e1} se o obchodn\u{00ed} dokument vyb\u{00ed}zej\u{00ed}c\u{00ed} k platb\u{011b}.\n\nPovinn\u{00e9} n\u{00e1}le\u{017e}itosti da\u{0148}ov\u{00e9}ho dokladu upravuje \u{00a7} 29 t\u{00e9}ho\u{017e} z\u{00e1}kona. Po p\u{0159}ijet\u{00ed} \u{00fa}hrady z\u{00e1}lohov\u{00e9} faktury vznik\u{00e1} povinnost vystavit \u{0159}\u{00e1}dn\u{00fd} da\u{0148}ov\u{00fd} doklad dle \u{00a7} 28 odst. 2.",
        },
        HelpTopicId::Dobropis => HelpTopic {
            title: "Dobropis (opravn\u{00fd} da\u{0148}ov\u{00fd} doklad)",
            simple: "Dobropis je opravn\u{00fd} doklad, kter\u{00fd} vystavujete, kdy\u{017e} pot\u{0159}ebujete sn\u{00ed}\u{017e}it \u{010d}\u{00e1}stku na ji\u{017e} vydan\u{00e9} faktu\u{0159}e. Typick\u{00e9} d\u{016f}vody: sleva, reklamace, chybn\u{011b} \u{00fa}\u{010d}tovan\u{00e1} \u{010d}\u{00e1}stka nebo vr\u{00e1}cen\u{00ed} zbo\u{017e}\u{00ed}.\n\nDobropis odkazuje na p\u{016f}vodn\u{00ed} fakturu a obsahuje z\u{00e1}pornou \u{010d}\u{00e1}stku. Po jeho vystaven\u{00ed} se sn\u{00ed}\u{017e}\u{00ed} va\u{0161}e da\u{0148}ov\u{00e9} z\u{00e1}vazky.".into(),
            legal: "Opravn\u{00fd} da\u{0148}ov\u{00fd} doklad upravuje \u{00a7} 42 z\u{00e1}kona \u{010d}. 235/2004 Sb. o DPH. Pl\u{00e1}tce je povinen vystavit opravn\u{00fd} da\u{0148}ov\u{00fd} doklad do 15 dn\u{016f} ode dne zji\u{0161}t\u{011b}n\u{00ed} skutecnost\u{00ed} rozhodn\u{00fd}ch pro proveden\u{00ed} opravy (\u{00a7} 42 odst. 2).\n\nOpravn\u{00fd} doklad mus\u{00ed} obsahovat d\u{016f}vod opravy, rozd\u{00ed}l mezi p\u{016f}vodn\u{00ed} a novou \u{010d}\u{00e1}stkou a odkaz na p\u{016f}vodn\u{00ed} da\u{0148}ov\u{00fd} doklad (\u{00a7} 45 odst. 1).",
        },
        HelpTopicId::VyrovnaniZalohy => HelpTopic {
            title: "Vyrovn\u{00e1}n\u{00ed} z\u{00e1}lohy",
            simple: "Po zaplacen\u{00ed} z\u{00e1}lohov\u{00e9} faktury (proformy) je t\u{0159}eba vystavit \u{0159}\u{00e1}dnou fakturu. Tato faktura obsahuje celkovou \u{010d}\u{00e1}stku za dodan\u{00e9} zbo\u{017e}\u{00ed} \u{010d}i slu\u{017e}by, od kter\u{00e9} se ode\u{010d}te ji\u{017e} uhrazen\u{00e1} z\u{00e1}loha.\n\nV\u{00fd}sledkem je doplatek, kter\u{00fd} z\u{00e1}kazn\u{00ed}k je\u{0161}t\u{011b} uhrad\u{00ed}, nebo nulov\u{00e1} \u{010d}\u{00e1}stka, pokud z\u{00e1}loha pokryla v\u{0161}e.".into(),
            legal: "Povinnost vystavit da\u{0148}ov\u{00fd} doklad po p\u{0159}ijet\u{00ed} \u{00fa}hrady vypl\u{00fd}v\u{00e1} z \u{00a7} 21 odst. 1 z\u{00e1}kona \u{010d}. 235/2004 Sb. o DPH. Dnem p\u{0159}ijet\u{00ed} \u{00fa}hrady vznik\u{00e1} povinnost p\u{0159}iznat da\u{0148} na v\u{00fd}stupu.\n\nP\u{0159}i vyrovn\u{00e1}n\u{00ed} se na \u{0159}\u{00e1}dn\u{00e9} faktu\u{0159}e uvede celkov\u{00e1} \u{010d}\u{00e1}stka pln\u{011b}n\u{00ed} a ode\u{010d}te se d\u{0159}\u{00ed}ve uhrazen\u{00e1} z\u{00e1}loha. Z\u{00e1}klad dan\u{011b} a DPH se vypo\u{010d}tou z celkov\u{00e9} \u{010d}\u{00e1}stky pln\u{011b}n\u{00ed}.",
        },
        HelpTopicId::IsdocExport => HelpTopic {
            title: "Export ISDOC",
            simple: "ISDOC je \u{010d}esk\u{00fd} standard pro elektronickou fakturaci. Soubor ve form\u{00e1}tu ISDOC (.isdoc) obsahuje v\u{0161}echna data faktury ve strojov\u{011b} \u{010d}iteln\u{00e9} podob\u{011b}.\n\nKdy\u{017e} po\u{0161}lete fakturu ve form\u{00e1}tu ISDOC, odb\u{011b}ratel\u{016f}v \u{00fa}\u{010d}etn\u{00ed} syst\u{00e9}m ji m\u{016f}\u{017e}e automaticky na\u{010d}\u{00ed}st bez ru\u{010d}n\u{00ed}ho p\u{0159}episov\u{00e1}n\u{00ed}.".into(),
            legal: "ISDOC (Information System Document) je \u{010d}esk\u{00fd} n\u{00e1}rodn\u{00ed} standard elektronick\u{00e9} fakturace definovan\u{00fd} ICT Uni\u{00ed}. Form\u{00e1}t je zalo\u{017e}en\u{00fd} na UN/CEFACT a je kompatibiln\u{00ed} s evropskou normou EN 16931.\n\nPou\u{017e}\u{00ed}v\u{00e1}n\u{00ed} elektronick\u{00fd}ch faktur upravuje \u{00a7} 26 odst. 3 a \u{00a7} 34 z\u{00e1}kona \u{010d}. 235/2004 Sb. Elektronick\u{00e1} faktura mus\u{00ed} b\u{00fd}t opat\u{0159}ena zaru\u{010d}en\u{00fd}mi prost\u{0159}edky pro ov\u{011b}\u{0159}en\u{00ed} p\u{016f}vodu a neporu\u{0161}enosti obsahu.",
        },
        HelpTopicId::DanovaKontrola => HelpTopic {
            title: "Da\u{0148}ov\u{00e1} kontrola n\u{00e1}klad\u{016f}",
            simple: "Da\u{0148}ov\u{00e1} kontrola n\u{00e1}klad\u{016f} je proces, kdy systematicky projdete sv\u{00e9} v\u{00fd}daje a ov\u{011b}\u{0159}\u{00ed}te, \u{017e}e ka\u{017e}d\u{00fd} n\u{00e1}klad je spr\u{00e1}vn\u{011b} dolo\u{017e}en, spr\u{00e1}vn\u{011b} za\u{0159}azen a da\u{0148}ov\u{011b} uznateln\u{00fd}.\n\nOzna\u{010d}en\u{00ed}m n\u{00e1}kladu jako \"zkontrolovan\u{00fd}\" si udr\u{017e}ujete p\u{0159}ehled o tom, kter\u{00e9} v\u{00fd}daje jste ji\u{017e} ov\u{011b}\u{0159}ili a kter\u{00e9} je\u{0161}t\u{011b} \u{010d}ekaj\u{00ed} na kontrolu.".into(),
            legal: "Da\u{0148}ov\u{011b} uznateln\u{00e9} n\u{00e1}klady jsou definov\u{00e1}ny v \u{00a7} 24-25 z\u{00e1}kona \u{010d}. 586/1992 Sb. o dan\u{00ed}ch z p\u{0159}\u{00ed}jm\u{016f}. Podnikatel je povinen prok\u{00e1}zat, \u{017e}e v\u{00fd}daj byl vynalo\u{017e}en na dosa\u{017e}en\u{00ed}, zaji\u{0161}t\u{011b}n\u{00ed} a udr\u{017e}en\u{00ed} zdaniteln\u{00fd}ch p\u{0159}\u{00ed}jm\u{016f}.\n\nSpr\u{00e1}vce dan\u{011b} m\u{016f}\u{017e}e v r\u{00e1}mci da\u{0148}ov\u{00e9} kontroly (\u{00a7} 85 z\u{00e1}kona \u{010d}. 280/2009 Sb.) po\u{017e}adovat prok\u{00e1}z\u{00e1}n\u{00ed} opr\u{00e1}vn\u{011b}nosti v\u{0161}ech uplatn\u{011b}n\u{00fd}ch n\u{00e1}klad\u{016f}. Pravideln\u{00e1} kontrola minimalizuje riziko doplacen\u{00ed} dan\u{011b}.",
        },
        HelpTopicId::OcrImport => HelpTopic {
            title: "Import z dokladu (OCR)",
            simple: "OCR (optick\u{00e9} rozpozn\u{00e1}v\u{00e1}n\u{00ed} znak\u{016f}) automaticky p\u{0159}e\u{010d}te text z nahran\u{00e9} faktury nebo \u{00fa}\u{010d}tenky. Sta\u{010d}\u{00ed} nahr\u{00e1}t soubor (PDF, JPG, PNG nebo WebP) a syst\u{00e9}m se pokus\u{00ed} rozpoznat dodavatele, \u{010d}\u{00e1}stku, datum a dal\u{0161}\u{00ed} \u{00fa}daje.\n\nRozpoznan\u{00e1} data m\u{016f}\u{017e}ete p\u{0159}ed ulo\u{017e}en\u{00ed}m zkontrolovat a upravit.".into(),
            legal: "Archivace da\u{0148}ov\u{00fd}ch doklad\u{016f} v elektronick\u{00e9} podob\u{011b} je upravena v \u{00a7} 35a z\u{00e1}kona \u{010d}. 235/2004 Sb. a \u{00a7} 31-32 z\u{00e1}kona \u{010d}. 563/1991 Sb. o \u{00fa}\u{010d}etnictv\u{00ed}. Elektronick\u{00e1} kopie mus\u{00ed} zachovat v\u{011b}rnost a \u{010d}itelnost p\u{016f}vodn\u{00ed}ho dokladu.\n\nPovinnost uchovat da\u{0148}ov\u{00e9} doklady je 10 let od konce zda\u{0148}ovac\u{00ed}ho obdob\u{00ed} (\u{00a7} 35 z\u{00e1}kona \u{010d}. 235/2004 Sb.).",
        },
        HelpTopicId::PlatebniPodminky => HelpTopic {
            title: "Platebn\u{00ed} podm\u{00ed}nky",
            simple: "Splatnost ve dnech ur\u{010d}uje, kolik dn\u{00ed} od vystaven\u{00ed} faktury m\u{00e1} z\u{00e1}kazn\u{00ed}k na zaplacen\u{00ed}. Tato hodnota se automaticky nastav\u{00ed} na nov\u{00fd}ch faktur\u{00e1}ch pro tohoto z\u{00e1}kazn\u{00ed}ka.\n\nB\u{011b}\u{017e}n\u{00e1} splatnost je 14 nebo 30 dn\u{00ed}. Pro st\u{00e1}l\u{00e9} z\u{00e1}kazn\u{00ed}ky m\u{016f}\u{017e}ete nastavit individu\u{00e1}ln\u{00ed} splatnost.".into(),
            legal: "Splatnost je smluvn\u{00ed} ujedn\u{00e1}n\u{00ed} dle \u{00a7} 1958-1964 z\u{00e1}kona \u{010d}. 89/2012 Sb. (ob\u{010d}ansk\u{00fd} z\u{00e1}kon\u{00ed}k). Pro obchodn\u{00ed} vztahy mezi podnikateli je maxim\u{00e1}ln\u{00ed} smluvn\u{00ed} splatnost 60 dn\u{00ed} dle \u{00a7} 1963a OZ.\n\nPro vztahy s ve\u{0159}ejn\u{00fd}m sektorem plat\u{00ed} maxim\u{00e1}ln\u{00ed} splatnost 30 dn\u{00ed} (\u{00a7} 1963 OZ). Del\u{0161}\u{00ed} splatnost je mo\u{017e}n\u{00e1} jen pokud to nen\u{00ed} v\u{016f}\u{010d}i v\u{011b}\u{0159}iteli hrub\u{011b} nespravedliv\u{00e9}.",
        },
        HelpTopicId::EmailSablony => HelpTopic {
            title: "\u{0160}ablony email\u{016f}",
            simple: "\u{0160}ablona emailu ur\u{010d}uje p\u{0159}edm\u{011b}t a text zpr\u{00e1}vy, kter\u{00e1} se ode\u{0161}le z\u{00e1}kazn\u{00ed}kovi spolu s fakturou. Pou\u{017e}ijte {invoice_number} a syst\u{00e9}m automaticky vlo\u{017e}\u{00ed} \u{010d}\u{00ed}slo faktury.\n\n\u{0160}ablonu nastav\u{00ed}te jednou a pak se pou\u{017e}ije pro v\u{0161}echny odeslan\u{00e9} faktury. P\u{0159}ed odesl\u{00e1}n\u{00ed}m m\u{016f}\u{017e}ete text je\u{0161}t\u{011b} upravit.".into(),
            legal: "Odesl\u{00e1}n\u{00ed} faktury emailem je b\u{011b}\u{017e}nou obchodn\u{00ed} prax\u{00ed}. Elektronick\u{00e9} doru\u{010d}en\u{00ed} da\u{0148}ov\u{00e9}ho dokladu je upraveno v \u{00a7} 34 z\u{00e1}kona \u{010d}. 235/2004 Sb. -- odb\u{011b}ratel mus\u{00ed} s elektronick\u{00fd}m doru\u{010d}en\u{00ed}m souhlasit.\n\nElektronick\u{00e1} faktura mus\u{00ed} spl\u{0148}ovat podm\u{00ed}nky pro ov\u{011b}\u{0159}en\u{00ed} p\u{016f}vodu a neporu\u{0161}enosti obsahu (\u{00a7} 34 odst. 1).",
        },
        HelpTopicId::OpakovaneFaktury => HelpTopic {
            title: "Opakovan\u{00e9} faktury",
            simple: "Opakovan\u{00e9} faktury jsou \u{0161}ablony, ze kter\u{00fd}ch se automaticky generuj\u{00ed} nov\u{00e9} faktury v pravideln\u{00fd}ch intervalech (m\u{011b}s\u{00ed}\u{010d}n\u{011b}, \u{010d}tvrtletn\u{011b}, ro\u{010d}n\u{011b}).\n\nHod\u{00ed} se pro pau\u{0161}\u{00e1}ln\u{00ed} slu\u{017e}by, n\u{00e1}jem, p\u{0159}edplatn\u{00e9} nebo jakoukoli pravidelnou fakturaci. \u{0160}ablona obsahuje z\u{00e1}kazn\u{00ed}ka, polo\u{017e}ky a frekvenci -- syst\u{00e9}m pak s\u{00e1}m vytvo\u{0159}\u{00ed} fakturu kdy\u{017e} p\u{0159}i\u{0161}el \u{010d}as.".into(),
            legal: "Opakovan\u{00e9} pln\u{011b}n\u{00ed} je upraveno v \u{00a7} 21 odst. 8 z\u{00e1}kona \u{010d}. 235/2004 Sb. o DPH. U opakuj\u{00ed}c\u{00ed}ho se pln\u{011b}n\u{00ed} se DUZP stanov\u{00ed} nejpozd\u{011b}ji posledn\u{00ed}m dnem zda\u{0148}ovac\u{00ed}ho obdob\u{00ed}.\n\nSmlouvy na opakovan\u{00e9} pln\u{011b}n\u{00ed} (n\u{00e1}jem, servisn\u{00ed} smlouvy) se \u{0159}\u{00ed}d\u{00ed} ustanoven\u{00ed}mi o z\u{00e1}vazkov\u{00e9}m pr\u{00e1}vu v ob\u{010d}ansk\u{00e9}m z\u{00e1}kon\u{00ed}ku (\u{00a7} 1724 a n\u{00e1}sl. z\u{00e1}kona \u{010d}. 89/2012 Sb.).",
        },
        HelpTopicId::KategorieNakladu => HelpTopic {
            title: "Kategorie n\u{00e1}klad\u{016f}",
            simple: "Kategorie pom\u{00e1}haj\u{00ed} t\u{0159}\u{00ed}dit n\u{00e1}klady podle typu (kancel\u{00e1}\u{0159}, cestovn\u{00e9}, slu\u{017e}by, materi\u{00e1}l apod.). Dob\u{0159}e rozt\u{0159}\u{00ed}d\u{011b}n\u{00e9} n\u{00e1}klady usnad\u{0148}uj\u{00ed} p\u{0159}ehled o v\u{00fd}daj\u{00ed}ch, p\u{0159}\u{00ed}pravu da\u{0148}ov\u{00e9}ho p\u{0159}izn\u{00e1}n\u{00ed} a komunikaci s \u{00fa}\u{010d}etn\u{00ed}m.\n\nM\u{016f}\u{017e}ete pou\u{017e}\u{00ed}t v\u{00fd}choz\u{00ed} kategorie nebo si vytvo\u{0159}it vlastn\u{00ed}.".into(),
            legal: "T\u{0159}\u{00ed}d\u{011b}n\u{00ed} n\u{00e1}klad\u{016f} podle kategori\u{00ed} nen\u{00ed} z\u{00e1}konem p\u{0159}edeps\u{00e1}no, ale vypl\u{00fd}v\u{00e1} z povinnosti v\u{00e9}st \u{00fa}\u{010d}etnictv\u{00ed} p\u{0159}ehledn\u{011b} a pr\u{016f}kazn\u{011b} (\u{00a7} 8 z\u{00e1}kona \u{010d}. 563/1991 Sb.).\n\nPro \u{00fa}\u{010d}ely da\u{0148}ov\u{00e9}ho p\u{0159}izn\u{00e1}n\u{00ed} je vhodn\u{00e9} \u{010d}lenit n\u{00e1}klady dle \u{00a7} 24 z\u{00e1}kona \u{010d}. 586/1992 Sb. (da\u{0148}ov\u{011b} uznateln\u{00e9}) a \u{00a7} 25 (neuznateln\u{00e9}), p\u{0159}\u{00ed}p. dle povahy v\u{00fd}daje pro spr\u{00e1}vn\u{00e9} vypln\u{011b}n\u{00ed} p\u{0159}\u{00ed}loh p\u{0159}izn\u{00e1}n\u{00ed}.",
        },
        HelpTopicId::DuplikaceFaktury => HelpTopic {
            title: "Duplikace faktury",
            simple: "Duplikace vytvo\u{0159}\u{00ed} novou fakturu jako kopii st\u{00e1}vaj\u{00ed}c\u{00ed}. Zkop\u{00ed}ruje se z\u{00e1}kazn\u{00ed}k, polo\u{017e}ky, zp\u{016f}sob platby a dal\u{0161}\u{00ed} nastaven\u{00ed}. Nov\u{00e1} faktura dostane nov\u{00e9} \u{010d}\u{00ed}slo a aktu\u{00e1}ln\u{00ed} datumy.\n\nHod\u{00ed} se, kdy\u{017e} vystavujete podobnou fakturu jako minule -- nemus\u{00ed}te v\u{0161}e vypl\u{0148}ovat znovu.".into(),
            legal: "Duplikovan\u{00e1} faktura je nov\u{00fd}, samostatn\u{00fd} da\u{0148}ov\u{00fd} doklad s vlastn\u{00ed}m po\u{0159}adov\u{00fd}m \u{010d}\u{00ed}slem dle \u{00a7} 29 z\u{00e1}kona \u{010d}. 235/2004 Sb. Jedn\u{00e1} se o zcela nez\u{00e1}visl\u{00fd} doklad, nikoliv o kopii p\u{016f}vodn\u{00ed}ho.\n\nPo\u{0159}adov\u{00e9} \u{010d}\u{00ed}slo mus\u{00ed} b\u{00fd}t unik\u{00e1}tn\u{00ed} v r\u{00e1}mci \u{010d}\u{00ed}seln\u{00e9} \u{0159}ady (\u{00a7} 29 odst. 1 p\u{00ed}sm. b).",
        },
        HelpTopicId::RocniDane => HelpTopic {
            title: "Ro\u{010d}n\u{00ed} dan\u{011b} a p\u{0159}ehledy OSV\u{010c}",
            simple: "Ro\u{010d}n\u{00ed} da\u{0148}ov\u{00e9} p\u{0159}izn\u{00e1}n\u{00ed} (DPFO) a p\u{0159}ehledy pro soci\u{00e1}ln\u{00ed} (\u{010c}SSZ) a zdravotn\u{00ed} poji\u{0161}\u{0165}ovnu (ZP). Aplikace spo\u{010d}\u{00ed}t\u{00e1} z\u{00e1}klad dan\u{011b} z faktur a n\u{00e1}klad\u{016f}, aplikuje sazby a slevy, a vygeneruje XML pro elektronick\u{00e9} pod\u{00e1}n\u{00ed}.".into(),
            legal: "Da\u{0148}ov\u{00e9} p\u{0159}izn\u{00e1}n\u{00ed} k dani z p\u{0159}\u{00ed}jm\u{016f} fyzick\u{00fd}ch osob (\u{00a7} 38g z\u{00e1}kona \u{010d}. 586/1992 Sb.). P\u{0159}ehled o p\u{0159}\u{00ed}jmech a v\u{00fd}daj\u{00ed}ch OSV\u{010c} pro \u{010c}SSZ (\u{00a7} 15 z\u{00e1}kona \u{010d}. 589/1992 Sb.) a pro zdravotn\u{00ed} poji\u{0161}\u{0165}ovnu (\u{00a7} 24 z\u{00e1}kona \u{010d}. 592/1992 Sb.).",
        },
        HelpTopicId::VymeroviciZaklad => HelpTopic {
            title: "Vym\u{011b}\u{0159}ovac\u{00ed} z\u{00e1}klad pro pojistn\u{00e9}",
            simple: "Vym\u{011b}\u{0159}ovac\u{00ed} z\u{00e1}klad je \u{010d}\u{00e1}stka, ze kter\u{00e9} se po\u{010d}\u{00ed}t\u{00e1} soci\u{00e1}ln\u{00ed} a zdravotn\u{00ed} pojistn\u{00e9}. Pro OSV\u{010c} je to 50 % ze z\u{00e1}kladu dan\u{011b} (p\u{0159}\u{00ed}jmy minus v\u{00fd}daje).\n\nExistuje minim\u{00e1}ln\u{00ed} vym\u{011b}\u{0159}ovac\u{00ed} z\u{00e1}klad -- i kdy\u{017e} m\u{00e1}te n\u{00ed}zk\u{00fd} zisk, zaplat\u{00ed}te pojistn\u{00e9} alespo\u{0148} z minima. U soci\u{00e1}ln\u{00ed}ho poji\u{0161}t\u{011b}n\u{00ed} je minimum dobrovoln\u{00e9} (pokud je hlavn\u{00ed} \u{010d}innost), u zdravotn\u{00ed}ho je povinn\u{00e9} v\u{017e}dy.".into(),
            legal: "Vym\u{011b}\u{0159}ovac\u{00ed} z\u{00e1}klad pro soci\u{00e1}ln\u{00ed} poji\u{0161}t\u{011b}n\u{00ed} OSV\u{010c}: 50 % z\u{00e1}kladu dan\u{011b} (\u{00a7} 5b z\u{00e1}kona \u{010d}. 589/1992 Sb.). Minim\u{00e1}ln\u{00ed} vym\u{011b}\u{0159}ovac\u{00ed} z\u{00e1}klad: 25 % pr\u{016f}m\u{011b}rn\u{00e9} mzdy pro hlavn\u{00ed} \u{010d}innost. Pro zdravotn\u{00ed} poji\u{0161}t\u{011b}n\u{00ed}: 50 % z\u{00e1}kladu dan\u{011b} (\u{00a7} 3a z\u{00e1}kona \u{010d}. 592/1992 Sb.), minim\u{00e1}ln\u{00ed} z\u{00e1}klad je 50 % pr\u{016f}m\u{011b}rn\u{00e9} mzdy (\u{00a7} 3a odst. 2).",
        },
        HelpTopicId::CasovyTest => HelpTopic {
            title: "\u{010c}asov\u{00fd} test 3 roky pro cenn\u{00e9} pap\u{00ed}ry",
            simple: "Pokud vlastn\u{00ed}te akcii, ETF nebo jin\u{00fd} cenn\u{00fd} pap\u{00ed}r d\u{00e9}le ne\u{017e} 3 roky a pak ho prod\u{00e1}te, zisk z prodeje je osvobozen\u{00fd} od dan\u{011b}. Tomu se \u{0159}\u{00ed}k\u{00e1} \"\u{010d}asov\u{00fd} test\".\n\nP\u{0159}\u{00ed}klad: Koup\u{00ed}te akcii v lednu 2022 a prod\u{00e1}te v \u{00fa}noru 2025 (d\u{00e9}le ne\u{017e} 3 roky) -- neplat\u{00ed}te \u{017e}\u{00e1}dnou da\u{0148} ze zisku. Pokud prod\u{00e1}te d\u{0159}\u{00ed}ve, zisk se mus\u{00ed} danit v r\u{00e1}mci \u{00a7} 10.".into(),
            legal: "Osvobozen\u{00ed} p\u{0159}\u{00ed}jm\u{016f} z prodeje cenn\u{00fd}ch pap\u{00ed}r\u{016f} po \u{010d}asov\u{00e9}m testu upravuje \u{00a7} 4 odst. 1 p\u{00ed}sm. w) z\u{00e1}kona \u{010d}. 586/1992 Sb. Doba dr\u{017e}en\u{00ed} mus\u{00ed} p\u{0159}ekro\u{010d}it 3 roky. Od 2025 se \u{010d}asov\u{00fd} test prodlu\u{017e}uje na 3 roky i pro kryptom\u{011b}ny (\u{00a7} 4 odst. 1 p\u{00ed}sm. x). Pro fondy kolektivn\u{00ed}ho investov\u{00e1}n\u{00ed} plat\u{00ed} rovn\u{011b}\u{017e} 3 roky (\u{00a7} 4 odst. 1 p\u{00ed}sm. w).",
        },
        HelpTopicId::PrehledCssz => HelpTopic {
            title: "P\u{0159}ehled OSV\u{010c} pro \u{010c}SSZ",
            simple: "P\u{0159}ehled pro \u{010c}eskou spr\u{00e1}vu soci\u{00e1}ln\u{00ed}ho zabezpe\u{010d}en\u{00ed} je ro\u{010d}n\u{00ed} formul\u{00e1}\u{0159}, ve kter\u{00e9}m vykazujete sv\u{00e9} p\u{0159}\u{00ed}jmy a v\u{00fd}daje z podnik\u{00e1}n\u{00ed}. \u{010c}SSZ z n\u{011b}j vypo\u{010d}\u{00ed}t\u{00e1} va\u{0161}e pojistn\u{00e9} a novou v\u{00fd}\u{0161}i m\u{011b}s\u{00ed}\u{010d}n\u{00ed}ch z\u{00e1}loh.\n\nP\u{0159}ehled se pod\u{00e1}v\u{00e1} do jednoho m\u{011b}s\u{00ed}ce po lh\u{016f}t\u{011b} pro pod\u{00e1}n\u{00ed} da\u{0148}ov\u{00e9}ho p\u{0159}izn\u{00e1}n\u{00ed}. Pokud v\u{00e1}m vy\u{0161}el doplatek, mus\u{00ed}te ho zaplatit do 8 dn\u{016f} od pod\u{00e1}n\u{00ed} p\u{0159}ehledu.".into(),
            legal: "Povinnost podat p\u{0159}ehled vypl\u{00fd}v\u{00e1} z \u{00a7} 15 z\u{00e1}kona \u{010d}. 589/1992 Sb. o pojistn\u{00e9}m na soci\u{00e1}ln\u{00ed} zabezpe\u{010d}en\u{00ed}. Lh\u{016f}ta: do jednoho m\u{011b}s\u{00ed}ce po lh\u{016f}t\u{011b} pro pod\u{00e1}n\u{00ed} da\u{0148}ov\u{00e9}ho p\u{0159}izn\u{00e1}n\u{00ed} (\u{00a7} 15 odst. 1). Doplatek pojistn\u{00e9}ho je splatn\u{00fd} do 8 dn\u{016f} po pod\u{00e1}n\u{00ed} p\u{0159}ehledu (\u{00a7} 14a odst. 2). Nov\u{00e1} v\u{00fd}\u{0161}e z\u{00e1}lohy plat\u{00ed} od m\u{011b}s\u{00ed}ce n\u{00e1}sleduj\u{00ed}c\u{00ed}ho po m\u{011b}s\u{00ed}ci pod\u{00e1}n\u{00ed} p\u{0159}ehledu.",
        },
        HelpTopicId::PrehledZp => HelpTopic {
            title: "P\u{0159}ehled OSV\u{010c} pro zdravotn\u{00ed} poji\u{0161}\u{0165}ovnu",
            simple: "P\u{0159}ehled pro zdravotn\u{00ed} poji\u{0161}\u{0165}ovnu je ro\u{010d}n\u{00ed} formul\u{00e1}\u{0159}, ve kter\u{00e9}m vykazujete sv\u{00e9} p\u{0159}\u{00ed}jmy a v\u{00fd}daje. Poji\u{0161}\u{0165}ovna z n\u{011b}j vypo\u{010d}\u{00ed}t\u{00e1} va\u{0161}e zdravotn\u{00ed} pojistn\u{00e9} a novou v\u{00fd}\u{0161}i m\u{011b}s\u{00ed}\u{010d}n\u{00ed}ch z\u{00e1}loh.\n\nP\u{0159}ehled se pod\u{00e1}v\u{00e1} do jednoho m\u{011b}s\u{00ed}ce po lh\u{016f}t\u{011b} pro pod\u{00e1}n\u{00ed} da\u{0148}ov\u{00e9}ho p\u{0159}izn\u{00e1}n\u{00ed}. Doplatek se plat\u{00ed} do 8 dn\u{016f} od pod\u{00e1}n\u{00ed}.".into(),
            legal: "Povinnost podat p\u{0159}ehled upravuje \u{00a7} 24 z\u{00e1}kona \u{010d}. 592/1992 Sb. o pojistn\u{00e9}m na v\u{0161}eobecn\u{00e9} zdravotn\u{00ed} poji\u{0161}t\u{011b}n\u{00ed}. Lh\u{016f}ta: do jednoho m\u{011b}s\u{00ed}ce po lh\u{016f}t\u{011b} pro pod\u{00e1}n\u{00ed} da\u{0148}ov\u{00e9}ho p\u{0159}izn\u{00e1}n\u{00ed} (\u{00a7} 24 odst. 2). Doplatek pojistn\u{00e9}ho je splatn\u{00fd} do 8 dn\u{016f} po pod\u{00e1}n\u{00ed} p\u{0159}ehledu (\u{00a7} 7 odst. 2). OSV\u{010c} p\u{0159}ehled pod\u{00e1}v\u{00e1} t\u{00e9} poji\u{0161}\u{0165}ovn\u{011b}, u kter\u{00e9} byla poji\u{0161}t\u{011b}na k 1. lednu p\u{0159}\u{00ed}slu\u{0161}n\u{00e9}ho roku.",
        },
        HelpTopicId::KapitalovePrijmyS8 => HelpTopic {
            title: "Kapit\u{00e1}lov\u{00e9} p\u{0159}\u{00ed}jmy (\u{00a7}8)",
            simple: "Kapit\u{00e1}lov\u{00e9} p\u{0159}\u{00ed}jmy zahrnuj\u{00ed} dividendy, \u{00fa}roky z vklad\u{016f}, kupony z dluhopis\u{016f} a v\u{00fd}platy z fond\u{016f}. V\u{011b}t\u{0161}ina t\u{011b}chto p\u{0159}\u{00ed}jm\u{016f} je zdan\u{011b}na sr\u{00e1}\u{017e}kovou dan\u{00ed} (15 %) p\u{0159}\u{00ed}mo u zdroje -- banka nebo broker da\u{0148} strhne automaticky.\n\nDo da\u{0148}ov\u{00e9}ho p\u{0159}izn\u{00e1}n\u{00ed} (\u{00a7}8) uv\u{00e1}d\u{00ed}te jen p\u{0159}\u{00ed}jmy, kter\u{00e9} nebyly zdan\u{011b}ny sr\u{00e1}\u{017e}kovou dan\u{00ed}, nebo zahrani\u{010d}n\u{00ed} dividendy, kde chcete uplatnit z\u{00e1}po\u{010d}et dan\u{011b}.".into(),
            legal: "Kapit\u{00e1}lov\u{00e9} p\u{0159}\u{00ed}jmy jsou definov\u{00e1}ny v \u{00a7} 8 z\u{00e1}kona \u{010d}. 586/1992 Sb. Sr\u{00e1}\u{017e}kov\u{00e1} da\u{0148} 15 % dle \u{00a7} 36 odst. 2 se uplat\u{00ed} u dividend, \u{00fa}rok\u{016f} a dal\u{0161}\u{00ed}ch p\u{0159}\u{00ed}jm\u{016f} z \u{00a7} 8. Zahrani\u{010d}n\u{00ed} kapit\u{00e1}lov\u{00e9} p\u{0159}\u{00ed}jmy se uv\u{00e1}d\u{011b}j\u{00ed} v p\u{0159}izn\u{00e1}n\u{00ed} a p\u{0159}\u{00ed}padn\u{00e1} zahrani\u{010d}n\u{00ed} sr\u{00e1}\u{017e}kov\u{00e1} da\u{0148} se zapo\u{010d}te dle smlouvy o zamezen\u{00ed} dvoj\u{00ed}ho zdan\u{011b}n\u{00ed} (\u{00a7} 38f).",
        },
        HelpTopicId::ObchodyCpS10 => HelpTopic {
            title: "Obchody s CP a kryptem (\u{00a7}10)",
            simple: "Zisky z prodeje cenn\u{00fd}ch pap\u{00ed}r\u{016f} (akci\u{00ed}, ETF, dluhopis\u{016f}) a kryptom\u{011b}n se dan\u{00ed} v r\u{00e1}mci \u{00a7} 10 jako \"ostatn\u{00ed} p\u{0159}\u{00ed}jmy\". Od p\u{0159}\u{00ed}jm\u{016f} z prodeje si ode\u{010d}tete nab\u{00fd}vac\u{00ed} cenu (po\u{0159}ad\u{00ed} FIFO) a poplatky.\n\nZdaniteln\u{00fd} je pouze zisk, a to jen pokud nep\u{0159}e\u{0161}lo 3 roky od n\u{00e1}kupu (\u{010d}asov\u{00fd} test). Pokud celkov\u{00e9} ostatn\u{00ed} p\u{0159}\u{00ed}jmy za rok nep\u{0159}es\u{00e1}hnou 100 000 K\u{010d}, m\u{016f}\u{017e}ou b\u{00fd}t tak\u{00e9} osvobozeny.".into(),
            legal: "P\u{0159}\u{00ed}jmy z prodeje cenn\u{00fd}ch pap\u{00ed}r\u{016f} a kryptom\u{011b}n upravuje \u{00a7} 10 odst. 1 p\u{00ed}sm. b) z\u{00e1}kona \u{010d}. 586/1992 Sb. V\u{00fd}dajem je nab\u{00fd}vac\u{00ed} cena dle \u{00a7} 10 odst. 4. Osvobozen\u{00ed} po \u{010d}asov\u{00e9}m testu 3 roky dle \u{00a7} 4 odst. 1 p\u{00ed}sm. w). Limit osvobozen\u{00ed} pro ostatn\u{00ed} p\u{0159}\u{00ed}jmy do 100 000 K\u{010d} dle \u{00a7} 10 odst. 3 p\u{00ed}sm. a). Ztr\u{00e1}ta z \u{00a7} 10 se nekompenzuje se zisky z \u{00a7} 7.",
        },
        HelpTopicId::NutnoPriznatDp => HelpTopic {
            title: "Kdy p\u{0159}iznat kapit\u{00e1}lov\u{00fd} p\u{0159}\u{00ed}jem v DP",
            simple: "Kapit\u{00e1}lov\u{00e9} p\u{0159}\u{00ed}jmy je t\u{0159}eba p\u{0159}iznat v da\u{0148}ov\u{00e9}m p\u{0159}izn\u{00e1}n\u{00ed}, pokud:\n\n- Zahrani\u{010d}n\u{00ed} dividendy nebyly zdan\u{011b}ny \u{010d}eskou sr\u{00e1}\u{017e}kovou dan\u{00ed}\n- Chcete zapo\u{010d}\u{00ed}st zahrani\u{010d}n\u{00ed} da\u{0148}\n- P\u{0159}\u{00ed}jem p\u{0159}esahuje limit pro osvobozen\u{00ed}\n- Zdrojem je P2P platforma \u{010d}i zahrani\u{010d}n\u{00ed} broker bez \u{010d}esk\u{00e9} sr\u{00e1}\u{017e}kov\u{00e9} dan\u{011b}\n\nP\u{0159}\u{00ed}jmy ji\u{017e} zdan\u{011b}n\u{00e9} \u{010d}eskou sr\u{00e1}\u{017e}kovou dan\u{00ed} (nap\u{0159}. CZ dividendy od \u{010d}esk\u{00e9}ho brokera) p\u{0159}izn\u{00e1}vat nemus\u{00ed}te.".into(),
            legal: "Povinnost p\u{0159}iznat kapit\u{00e1}lov\u{00fd} p\u{0159}\u{00ed}jem vypl\u{00fd}v\u{00e1} z \u{00a7} 8 a \u{00a7} 38g z\u{00e1}kona \u{010d}. 586/1992 Sb. P\u{0159}\u{00ed}jmy zdan\u{011b}n\u{00e9} sr\u{00e1}\u{017e}kovou dan\u{00ed} dle \u{00a7} 36 se do z\u{00e1}kladu dan\u{011b} nezahrnuj\u{00ed} (\u{00a7} 36 odst. 7), pokud se poplatn\u{00ed}k nerozhodne je zahrnout (\u{00a7} 36 odst. 7 v\u{011b}ta druh\u{00e1}). Zahrani\u{010d}n\u{00ed} p\u{0159}\u{00ed}jmy se uv\u{00e1}d\u{011b}j\u{00ed} v\u{017e}dy, z\u{00e1}po\u{010d}et dan\u{011b} dle \u{00a7} 38f a p\u{0159}\u{00ed}slu\u{0161}n\u{00e9} smlouvy o zamezen\u{00ed} dvoj\u{00ed}ho zdan\u{011b}n\u{00ed}.",
        },
        HelpTopicId::DoplatekPreplatek => HelpTopic {
            title: "Doplatek vs p\u{0159}eplatek",
            simple: "V\u{00fd}sledek da\u{0148}ov\u{00e9}ho p\u{0159}izn\u{00e1}n\u{00ed} je bu\u{010f} doplatek, nebo p\u{0159}eplatek:\n\n- Doplatek: va\u{0161}e da\u{0148} je vy\u{0161}\u{0161}\u{00ed} ne\u{017e} zaplacen\u{00e9} z\u{00e1}lohy -- rozd\u{00ed}l mus\u{00ed}te doplatit\n- P\u{0159}eplatek: zaplatili jste na z\u{00e1}loh\u{00e1}ch v\u{00ed}ce, ne\u{017e} \u{010d}inila va\u{0161}e da\u{0148} -- st\u{00e1}t v\u{00e1}m rozd\u{00ed}l vr\u{00e1}t\u{00ed}\n\nDoplatek je splatn\u{00fd} do lh\u{016f}ty pro pod\u{00e1}n\u{00ed} da\u{0148}ov\u{00e9}ho p\u{0159}izn\u{00e1}n\u{00ed}. O p\u{0159}eplatek mus\u{00ed}te po\u{017e}\u{00e1}dat (formul\u{00e1}\u{0159} \"\u{017d}\u{00e1}dost o vr\u{00e1}cen\u{00ed} p\u{0159}eplatku\").".into(),
            legal: "Splatnost dan\u{011b} z p\u{0159}\u{00ed}jm\u{016f} upravuje \u{00a7} 135 z\u{00e1}kona \u{010d}. 280/2009 Sb. (da\u{0148}ov\u{00fd} \u{0159}\u{00e1}d) -- da\u{0148} je splatn\u{00e1} v posledn\u{00ed} den lh\u{016f}ty pro pod\u{00e1}n\u{00ed} p\u{0159}izn\u{00e1}n\u{00ed}. P\u{0159}eplatek na dani vrac\u{00ed} spr\u{00e1}vce dan\u{011b} na z\u{00e1}klad\u{011b} \u{017e}\u{00e1}dosti do 30 dn\u{016f} (\u{00a7} 155 odst. 3 da\u{0148}ov\u{00e9}ho \u{0159}\u{00e1}du). P\u{0159}eplatek men\u{0161}\u{00ed} ne\u{017e} 200 K\u{010d} se nevrac\u{00ed} (\u{00a7} 155 odst. 2).",
        },
        HelpTopicId::SrazenaDan => HelpTopic {
            title: "Sr\u{00e1}\u{017e}en\u{00e1} da\u{0148} z kapit\u{00e1}lu",
            simple: "Sr\u{00e1}\u{017e}kov\u{00e1} da\u{0148} je da\u{0148}, kterou za v\u{00e1}s strhne banka nebo broker je\u{0161}t\u{011b} p\u{0159}ed v\u{00fd}platou. U \u{010d}esk\u{00fd}ch dividend a \u{00fa}rok\u{016f} je to 15 %. Vy obdr\u{017e}\u{00ed}te \u{010d}\u{00e1}stku ji\u{017e} po zdan\u{011b}n\u{00ed}.\n\nP\u{0159}\u{00ed}jem zdan\u{011b}n\u{00fd} sr\u{00e1}\u{017e}kovou dan\u{00ed} nemus\u{00ed}te uv\u{00e1}d\u{011b}t v da\u{0148}ov\u{00e9}m p\u{0159}izn\u{00e1}n\u{00ed} -- da\u{0148} je ji\u{017e} vypo\u{0159}\u{00e1}d\u{00e1}na. V\u{00fd}jimkou jsou zahrani\u{010d}n\u{00ed} dividendy, kde m\u{016f}\u{017e}e b\u{00fd}t sr\u{00e1}\u{017e}kov\u{00e1} da\u{0148} jin\u{00e1} a chcete ji zapo\u{010d}\u{00ed}st.".into(),
            legal: "Sr\u{00e1}\u{017e}kov\u{00e1} da\u{0148} je upravena v \u{00a7} 36 z\u{00e1}kona \u{010d}. 586/1992 Sb. Sazba 15 % se uplat\u{00ed} u dividend, \u{00fa}rok\u{016f} z vklad\u{016f}, \u{00fa}rok\u{016f} z dluhopis\u{016f} a dal\u{0161}\u{00ed}ch p\u{0159}\u{00ed}jm\u{016f} z \u{00a7} 8 (\u{00a7} 36 odst. 2). Pl\u{00e1}tcem sr\u{00e1}\u{017e}kov\u{00e9} dan\u{011b} je vypl\u{00e1}citel p\u{0159}\u{00ed}jmu (\u{00a7} 38d), kter\u{00fd} da\u{0148} sraz\u{00ed} a odvede do konce m\u{011b}s\u{00ed}ce n\u{00e1}sleduj\u{00ed}c\u{00ed}ho po m\u{011b}s\u{00ed}ci sra\u{017e}en\u{00ed}.",
        },
        HelpTopicId::KurzCnb => HelpTopic {
            title: "Kurz \u{010c}NB pro p\u{0159}epo\u{010d}et",
            simple: "Zahrani\u{010d}n\u{00ed} p\u{0159}\u{00ed}jmy a v\u{00fd}daje se pro da\u{0148}ov\u{00e9} \u{00fa}\u{010d}ely p\u{0159}epo\u{010d}\u{00ed}t\u{00e1}vaj\u{00ed} na \u{010d}esk\u{00e9} koruny kurzem \u{010c}NB. Pou\u{017e}\u{00ed}v\u{00e1} se kurz platn\u{00fd} v den uskute\u{010d}n\u{011b}n\u{00ed} transakce (den obchodu, den v\u{00fd}platy dividendy).\n\nAplikace pou\u{017e}\u{00ed}v\u{00e1} devizov\u{00fd} kurz \u{010c}NB. U m\u{011b}n, kter\u{00e9} \u{010c}NB neuv\u{00e1}d\u{00ed} p\u{0159}\u{00ed}mo, se pou\u{017e}ije k\u{0159}\u{00ed}\u{017e}ov\u{00fd} kurz p\u{0159}es USD.".into(),
            legal: "P\u{0159}epo\u{010d}et ciz\u{00ed}ho kurzu upravuje \u{00a7} 38 z\u{00e1}kona \u{010d}. 586/1992 Sb. Poplatn\u{00ed}k pou\u{017e}ije jednotn\u{00fd} kurz stanoven\u{00fd} GF\u{0158} (ro\u{010d}n\u{00ed} pr\u{016f}m\u{011b}rn\u{00fd} kurz) nebo kurz devizov\u{00e9}ho trhu \u{010c}NB platn\u{00fd} v den uskute\u{010d}n\u{011b}n\u{00ed} transakce. Jednotn\u{00fd} kurz vyd\u{00e1}v\u{00e1} GF\u{0158} v pokynu po skon\u{010d}en\u{00ed} roku. Pro \u{00fa}\u{010d}ely \u{00a7} 10 se b\u{011b}\u{017e}n\u{011b} pou\u{017e}\u{00ed}v\u{00e1} denn\u{00ed} kurz \u{010c}NB.",
        },
        HelpTopicId::NovaZaloha => HelpTopic {
            title: "Nov\u{00e1} m\u{011b}s\u{00ed}\u{010d}n\u{00ed} z\u{00e1}loha",
            simple: "Po pod\u{00e1}n\u{00ed} p\u{0159}ehledu \u{010c}SSZ a ZP v\u{00e1}m poji\u{0161}\u{0165}ovna vypo\u{010d}\u{00ed}t\u{00e1} novou v\u{00fd}\u{0161}i m\u{011b}s\u{00ed}\u{010d}n\u{00ed} z\u{00e1}lohy na dal\u{0161}\u{00ed} obdob\u{00ed}. V\u{00fd}\u{0161}e z\u{00e1}lohy se odv\u{00ed}j\u{00ed} od va\u{0161}ich p\u{0159}\u{00ed}jm\u{016f} v minul\u{00e9}m roce.\n\nPokud jste m\u{011b}li vy\u{0161}\u{0161}\u{00ed} p\u{0159}\u{00ed}jmy, z\u{00e1}lohy se zv\u{00fd}\u{0161}\u{00ed}. Pokud ni\u{017e}\u{0161}\u{00ed}, sn\u{00ed}\u{017e}\u{00ed} se (ale ne pod z\u{00e1}konn\u{00e9} minimum). Nov\u{00e1} z\u{00e1}loha plat\u{00ed} od m\u{011b}s\u{00ed}ce n\u{00e1}sleduj\u{00ed}c\u{00ed}ho po pod\u{00e1}n\u{00ed} p\u{0159}ehledu.".into(),
            legal: "Z\u{00e1}lohy na soci\u{00e1}ln\u{00ed} poji\u{0161}t\u{011b}n\u{00ed}: \u{00a7} 14a z\u{00e1}kona \u{010d}. 589/1992 Sb. Nov\u{00e1} v\u{00fd}\u{0161}e z\u{00e1}lohy = 1/12 ro\u{010d}n\u{00ed}ho pojistn\u{00e9}ho. Minim\u{00e1}ln\u{00ed} z\u{00e1}loha se odv\u{00ed}j\u{00ed} od pr\u{016f}m\u{011b}rn\u{00e9} mzdy. Plat\u{00ed} od m\u{011b}s\u{00ed}ce n\u{00e1}sleduj\u{00ed}c\u{00ed}ho po m\u{011b}s\u{00ed}ci pod\u{00e1}n\u{00ed} p\u{0159}ehledu. Z\u{00e1}lohy na zdravotn\u{00ed} poji\u{0161}t\u{011b}n\u{00ed}: \u{00a7} 7 z\u{00e1}kona \u{010d}. 592/1992 Sb. Minim\u{00e1}ln\u{00ed} z\u{00e1}loha je 50 % z minim\u{00e1}ln\u{00ed}ho vym\u{011b}\u{0159}ovac\u{00ed}ho z\u{00e1}kladu.",
        },
        HelpTopicId::FifoPrepocet => HelpTopic {
            title: "FIFO metoda pro nab\u{00fd}vac\u{00ed} cenu",
            simple: "FIFO (First In, First Out) je metoda pro ur\u{010d}en\u{00ed} nab\u{00fd}vac\u{00ed} ceny p\u{0159}i prodeji cenn\u{00fd}ch pap\u{00ed}r\u{016f}. Znamen\u{00e1}, \u{017e}e p\u{0159}i prodeji se jako prvn\u{00ed} \"spot\u{0159}ebuj\u{00ed}\" nejstar\u{0161}\u{00ed} nakoupen\u{00e9} kusy.\n\nP\u{0159}\u{00ed}klad: Koupili jste 10 ks za 100 K\u{010d} a pak 10 ks za 150 K\u{010d}. Pokud prod\u{00e1}te 10 ks, nab\u{00fd}vac\u{00ed} cena bude 100 K\u{010d} (pou\u{017e}ij\u{00ed} se prvn\u{00ed} nakoupen\u{00e9} kusy).\n\nFIFO metoda je pro OSV\u{010c} jedin\u{00e1} povolen\u{00e1} metoda.".into(),
            legal: "FIFO metoda je jedin\u{00e1} p\u{0159}\u{00ed}pustn\u{00e1} metoda oce\u{0148}ov\u{00e1}n\u{00ed} pro fyzick\u{00e9} osoby p\u{0159}i prodeji cenn\u{00fd}ch pap\u{00ed}r\u{016f} dle \u{00a7} 10 odst. 4 z\u{00e1}kona \u{010d}. 586/1992 Sb. a pokynu GF\u{0158}-D-22. P\u{0159}i FIFO se p\u{0159}i\u{0159}ad\u{00ed} v\u{00fd}daj k p\u{0159}\u{00ed}mo identifikovateln\u{00e9}mu n\u{00e1}kupu, nebo se pou\u{017e}ije nejstar\u{0161}\u{00ed} nep\u{0159}i\u{0159}azen\u{00fd} n\u{00e1}kup. N\u{00e1}klady na poplatky brokera jsou sou\u{010d}\u{00e1}st\u{00ed} nab\u{00fd}vac\u{00ed} ceny.",
        },
        // ── Dynamic topics ──────────────────────────────────────────────
        HelpTopicId::PausalniVydaje => {
            let simple = if let Some(tc) = tc {
                let year = year_from_tc(tc);
                let mut caps: Vec<(i32, Amount)> = tc.flat_rate_caps.iter().map(|(&k, &v)| (k, v)).collect();
                caps.sort_by_key(|b| std::cmp::Reverse(b.0));
                let lines: Vec<String> = caps.iter().map(|(pct, cap)| format!("- {} %: max {}", pct, fmt_czk(*cap))).collect();
                format!(
                    "Pau\u{0161}\u{00e1}ln\u{00ed} v\u{00fd}daje jsou zjednodu\u{0161}en\u{00fd} zp\u{016f}sob uplat\u{0148}ov\u{00e1}n\u{00ed} n\u{00e1}klad\u{016f} -- m\u{00ed}sto evidov\u{00e1}n\u{00ed} ka\u{017e}d\u{00e9}ho v\u{00fd}daje si ode\u{010d}tete procento z p\u{0159}\u{00ed}jm\u{016f}. Procenta a maxima pro rok {}:\n\n{}\n\nPau\u{0161}\u{00e1}ln\u{00ed} v\u{00fd}daje se hod\u{00ed}, pokud m\u{00e1}te n\u{00ed}zk\u{00e9} skute\u{010d}n\u{00e9} n\u{00e1}klady. Pozor: p\u{0159}i pau\u{0161}\u{00e1}ln\u{00ed}ch v\u{00fd}daj\u{00ed}ch nelze uplatnit slevu na man\u{017e}ela/ku ani da\u{0148}ov\u{00e9} zv\u{00fd}hodn\u{011b}n\u{00ed} na d\u{011b}ti.",
                    year,
                    lines.join("\n"),
                )
            } else {
                "Pau\u{0161}\u{00e1}ln\u{00ed} v\u{00fd}daje jsou zjednodu\u{0161}en\u{00fd} zp\u{016f}sob uplat\u{0148}ov\u{00e1}n\u{00ed} n\u{00e1}klad\u{016f} -- m\u{00ed}sto evidov\u{00e1}n\u{00ed} ka\u{017e}d\u{00e9}ho v\u{00fd}daje si ode\u{010d}tete procento z p\u{0159}\u{00ed}jm\u{016f}. Ka\u{017e}d\u{00e9} procento m\u{00e1} ro\u{010d}n\u{00ed} strop, kter\u{00fd} se m\u{016f}\u{017e}e li\u{0161}it podle roku.\n\nPau\u{0161}\u{00e1}ln\u{00ed} v\u{00fd}daje se hod\u{00ed}, pokud m\u{00e1}te n\u{00ed}zk\u{00e9} skute\u{010d}n\u{00e9} n\u{00e1}klady. Pozor: p\u{0159}i pau\u{0161}\u{00e1}ln\u{00ed}ch v\u{00fd}daj\u{00ed}ch nelze uplatnit slevu na man\u{017e}ela/ku ani da\u{0148}ov\u{00e9} zv\u{00fd}hodn\u{011b}n\u{00ed} na d\u{011b}ti.".into()
            };
            HelpTopic {
                title: "Pau\u{0161}\u{00e1}ln\u{00ed} v\u{00fd}daje",
                simple,
                legal: "Pau\u{0161}\u{00e1}ln\u{00ed} v\u{00fd}daje (v\u{00fd}daje procentem z p\u{0159}\u{00ed}jm\u{016f}) upravuje \u{00a7} 7 odst. 7 z\u{00e1}kona \u{010d}. 586/1992 Sb. Sazby: 80 % (zem\u{011b}d\u{011b}lstv\u{00ed}, \u{0159}emesla), 60 % (\u{017e}ivnost voln\u{00e1}), 40 % (svobodn\u{00e1} povol\u{00e1}n\u{00ed}), 30 % (n\u{00e1}jem). Stropy se m\u{011b}n\u{00ed} podle roku. P\u{0159}i pau\u{0161}\u{00e1}ln\u{00ed}ch v\u{00fd}daj\u{00ed}ch nelze uplatnit slevu na man\u{017e}ela (\u{00a7} 35ca) ani da\u{0148}ov\u{00e9} zv\u{00fd}hodn\u{011b}n\u{00ed} na d\u{011b}ti (\u{00a7} 35c odst. 9).",
            }
        }
        HelpTopicId::Dan1523 => {
            let simple = if let Some(tc) = tc {
                let year = year_from_tc(tc);
                let threshold = fmt_czk(tc.progressive_threshold);
                format!(
                    "Da\u{0148} z p\u{0159}\u{00ed}jm\u{016f} fyzick\u{00fd}ch osob m\u{00e1} dv\u{011b} sazby:\n\n- 15 % ze z\u{00e1}kladu dan\u{011b} do {threshold}\n- 23 % z \u{010d}\u{00e1}sti z\u{00e1}kladu dan\u{011b} nad {threshold}\n\nPr\u{00e1}h {threshold} odpov\u{00ed}d\u{00e1} 48n\u{00e1}sobku pr\u{016f}m\u{011b}rn\u{00e9} mzdy pro rok {year}. V\u{011b}t\u{0161}ina OSV\u{010c} se vejde do 15% p\u{00e1}sma."
                )
            } else {
                "Da\u{0148} z p\u{0159}\u{00ed}jm\u{016f} fyzick\u{00fd}ch osob m\u{00e1} dv\u{011b} sazby:\n\n- 15 % ze z\u{00e1}kladu dan\u{011b} do z\u{00e1}konem stanoven\u{00e9}ho prahu\n- 23 % z \u{010d}\u{00e1}sti z\u{00e1}kladu dan\u{011b} nad tento pr\u{00e1}h\n\nPr\u{00e1}h odpov\u{00ed}d\u{00e1} 48n\u{00e1}sobku pr\u{016f}m\u{011b}rn\u{00e9} mzdy a m\u{011b}n\u{00ed} se ka\u{017e}d\u{00fd} rok. V\u{011b}t\u{0161}ina OSV\u{010c} se vejde do 15% p\u{00e1}sma.".into()
            };
            HelpTopic {
                title: "Sazba dan\u{011b} 15 % a 23 %",
                simple,
                legal: "Sazby dan\u{011b} z p\u{0159}\u{00ed}jm\u{016f} fyzick\u{00fd}ch osob upravuje \u{00a7} 16 z\u{00e1}kona \u{010d}. 586/1992 Sb. Z\u{00e1}kladn\u{00ed} sazba 15 % a solid\u{00e1}rn\u{00ed} sazba 23 % z \u{010d}\u{00e1}sti z\u{00e1}kladu dan\u{011b} p\u{0159}esahuj\u{00ed}c\u{00ed} 48n\u{00e1}sobek pr\u{016f}m\u{011b}rn\u{00e9} mzdy (\u{00a7} 16 odst. 2). Pr\u{016f}m\u{011b}rn\u{00e1} mzda se stanov\u{00ed} dle \u{00a7} 21g.",
            }
        }
        HelpTopicId::SlevaNaPoplatnika => {
            let simple = if let Some(tc) = tc {
                let year = year_from_tc(tc);
                let annual = fmt_czk(tc.basic_credit);
                let monthly = fmt_czk(Amount::new(tc.basic_credit.to_czk() as i64 / 12, 0));
                let threshold = fmt_czk(Amount::new((tc.basic_credit.to_czk() / 0.15) as i64, 0));
                format!(
                    "Z\u{00e1}kladn\u{00ed} sleva na poplatn\u{00ed}ka je \u{010d}\u{00e1}stka, kterou si ka\u{017e}d\u{00fd} automaticky ode\u{010d}te od vypo\u{010d}ten\u{00e9} dan\u{011b}. Pro rok {year} \u{010d}in\u{00ed} {annual} ro\u{010d}n\u{011b} ({monthly} m\u{011b}s\u{00ed}\u{010d}n\u{011b}).\n\nD\u{00ed}ky t\u{00e9}to slev\u{011b} neplat\u{00ed}te da\u{0148} z prvn\u{00ed}ch cca {threshold} zisku. Sleva se uplat\u{00ed} v\u{017e}dy v pln\u{00e9} v\u{00fd}\u{0161}i -- neproporcionalizuje se podle m\u{011b}s\u{00ed}c\u{016f}."
                )
            } else {
                "Z\u{00e1}kladn\u{00ed} sleva na poplatn\u{00ed}ka je \u{010d}\u{00e1}stka, kterou si ka\u{017e}d\u{00fd} automaticky ode\u{010d}te od vypo\u{010d}ten\u{00e9} dan\u{011b}. Konkr\u{00e9}tn\u{00ed} v\u{00fd}\u{0161}e z\u{00e1}vis\u{00ed} na zda\u{0148}ovac\u{00ed}m obdob\u{00ed}.\n\nD\u{00ed}ky t\u{00e9}to slev\u{011b} neplat\u{00ed}te da\u{0148} z ur\u{010d}it\u{00e9} \u{010d}\u{00e1}sti zisku. Sleva se uplat\u{00ed} v\u{017e}dy v pln\u{00e9} v\u{00fd}\u{0161}i -- neproporcionalizuje se podle m\u{011b}s\u{00ed}c\u{016f}.".into()
            };
            HelpTopic {
                title: "Z\u{00e1}kladn\u{00ed} sleva na dani",
                simple,
                legal: "Z\u{00e1}kladn\u{00ed} sleva na poplatn\u{00ed}ka je upravena v \u{00a7} 35ba odst. 1 p\u{00ed}sm. a) z\u{00e1}kona \u{010d}. 586/1992 Sb. Tuto slevu uplat\u{0148}uje ka\u{017e}d\u{00fd} poplatn\u{00ed}k bez ohledu na v\u{00fd}\u{0161}i p\u{0159}\u{00ed}jm\u{016f}. Na rozd\u{00ed}l od ostatn\u{00ed}ch slev se neproporcionalizuje a uplat\u{0148}uje se v\u{017e}dy v pln\u{00e9} ro\u{010d}n\u{00ed} v\u{00fd}\u{0161}i.",
            }
        }
        HelpTopicId::ZvyhodneniNaDeti => {
            let simple = if let Some(tc) = tc {
                let year = year_from_tc(tc);
                format!(
                    "Da\u{0148}ov\u{00e9} zv\u{00fd}hodn\u{011b}n\u{00ed} na d\u{011b}ti je \u{010d}\u{00e1}stka, kterou si ode\u{010d}tete od dan\u{011b} za ka\u{017e}d\u{00e9} vy\u{017e}ivovan\u{00e9} d\u{00ed}t\u{011b}. Ro\u{010d}n\u{00ed} \u{010d}\u{00e1}stky ({year}):\n\n- 1. d\u{00ed}t\u{011b}: {}\n- 2. d\u{00ed}t\u{011b}: {}\n- 3. a dal\u{0161}\u{00ed}: {}\n\nPokud je d\u{00ed}t\u{011b} dr\u{017e}itelem ZTP/P, \u{010d}\u{00e1}stka se zdvojn\u{00e1}sobuje. Zv\u{00fd}hodn\u{011b}n\u{00ed} m\u{016f}\u{017e}e vytvo\u{0159}it \"da\u{0148}ov\u{00fd} bonus\" -- pokud je vy\u{0161}\u{0161}\u{00ed} ne\u{017e} va\u{0161}e da\u{0148}, st\u{00e1}t v\u{00e1}m rozd\u{00ed}l vr\u{00e1}t\u{00ed} (max {}/rok).",
                    fmt_czk(tc.child_benefit_1),
                    fmt_czk(tc.child_benefit_2),
                    fmt_czk(tc.child_benefit_3_plus),
                    fmt_czk(tc.max_child_bonus),
                )
            } else {
                "Da\u{0148}ov\u{00e9} zv\u{00fd}hodn\u{011b}n\u{00ed} na d\u{011b}ti je \u{010d}\u{00e1}stka, kterou si ode\u{010d}tete od dan\u{011b} za ka\u{017e}d\u{00e9} vy\u{017e}ivovan\u{00e9} d\u{00ed}t\u{011b}. Konkr\u{00e9}tn\u{00ed} v\u{00fd}\u{0161}e se li\u{0161}\u{00ed} podle po\u{0159}ad\u{00ed} d\u{00ed}t\u{011b}te a zda\u{0148}ovac\u{00ed}ho obdob\u{00ed}.\n\nPokud je d\u{00ed}t\u{011b} dr\u{017e}itelem ZTP/P, \u{010d}\u{00e1}stka se zdvojn\u{00e1}sobuje. Zv\u{00fd}hodn\u{011b}n\u{00ed} m\u{016f}\u{017e}e vytvo\u{0159}it \"da\u{0148}ov\u{00fd} bonus\" -- pokud je vy\u{0161}\u{0161}\u{00ed} ne\u{017e} va\u{0161}e da\u{0148}, st\u{00e1}t v\u{00e1}m rozd\u{00ed}l vr\u{00e1}t\u{00ed}.".into()
            };
            HelpTopic {
                title: "Da\u{0148}ov\u{00e9} zv\u{00fd}hodn\u{011b}n\u{00ed} na d\u{011b}ti",
                simple,
                legal: "Da\u{0148}ov\u{00e9} zv\u{00fd}hodn\u{011b}n\u{00ed} na vy\u{017e}ivovan\u{00e9} d\u{00ed}t\u{011b} upravuje \u{00a7} 35c z\u{00e1}kona \u{010d}. 586/1992 Sb. \u{010c}\u{00e1}stky se m\u{011b}n\u{00ed} podle roku. U d\u{00ed}t\u{011b}te s ZTP/P se \u{010d}\u{00e1}stky zdvojn\u{00e1}sobuj\u{00ed} (\u{00a7} 35c odst. 1). Maxim\u{00e1}ln\u{00ed} ro\u{010d}n\u{00ed} da\u{0148}ov\u{00fd} bonus je stanoven v \u{00a7} 35c odst. 3. Zv\u{00fd}hodn\u{011b}n\u{00ed} nelze uplatnit p\u{0159}i pau\u{0161}\u{00e1}ln\u{00ed}ch v\u{00fd}daj\u{00ed}ch (\u{00a7} 35c odst. 9).",
            }
        }
        HelpTopicId::MesiceProporcializace => HelpTopic {
            title: "Proporcionalizace slev podle m\u{011b}s\u{00ed}c\u{016f}",
            simple: "N\u{011b}kter\u{00e9} slevy a zv\u{00fd}hodn\u{011b}n\u{00ed} se po\u{010d}\u{00ed}taj\u{00ed} v pom\u{011b}rn\u{00e9} v\u{00fd}\u{0161}i podle po\u{010d}tu m\u{011b}s\u{00ed}c\u{016f}, po kter\u{00e9} podm\u{00ed}nka platila. Nap\u{0159}. pokud jste se o\u{017e}enili v \u{010d}ervnu, slevu na man\u{017e}ela/ku uplat\u{00ed}te za 7 m\u{011b}s\u{00ed}c\u{016f} (\u{010d}erven-prosinec).\n\nStejn\u{011b} to funguje u d\u{011b}t\u{00ed} -- pokud se d\u{00ed}t\u{011b} narodilo v \u{0159}\u{00ed}jnu, zv\u{00fd}hodn\u{011b}n\u{00ed} uplat\u{00ed}te za 3 m\u{011b}s\u{00ed}ce. Rozhoduje stav na za\u{010d}\u{00e1}tku m\u{011b}s\u{00ed}ce.".into(),
            legal: "Proporcionalizace slev je upravena v \u{00a7} 35ba odst. 3 a \u{00a7} 35c odst. 8 z\u{00e1}kona \u{010d}. 586/1992 Sb. Sleva na man\u{017e}ela/ku a da\u{0148}ov\u{00e9} zv\u{00fd}hodn\u{011b}n\u{00ed} na d\u{00ed}t\u{011b} se uplat\u{0148}uj\u{00ed} v pom\u{011b}rn\u{00e9} v\u{00fd}\u{0161}i odpov\u{00ed}daj\u{00ed}c\u{00ed} po\u{010d}tu kalend\u{00e1}\u{0159}n\u{00ed}ch m\u{011b}s\u{00ed}c\u{016f}, na jejich\u{017e} po\u{010d}\u{00e1}tku byly spln\u{011b}ny podm\u{00ed}nky pro uplatn\u{011b}n\u{00ed}.",
        },
        HelpTopicId::NezdanitelneOdpocty => {
            let simple = if let Some(tc) = tc {
                format!(
                    "Nezdaniteln\u{00e9} \u{010d}\u{00e1}sti z\u{00e1}kladu dan\u{011b} jsou \u{010d}\u{00e1}stky, kter\u{00e9} si ode\u{010d}tete od z\u{00e1}kladu dan\u{011b} P\u{0158}ED v\u{00fd}po\u{010d}tem dan\u{011b} (na rozd\u{00ed}l od slev, kter\u{00e9} se ode\u{010d}\u{00ed}taj\u{00ed} od dan\u{011b} samotn\u{00e9}). Pat\u{0159}\u{00ed} sem:\n\n- \u{00da}roky z hypot\u{00e9}ky (max {}/rok)\n- Penzijn\u{00ed} spo\u{0159}en\u{00ed} (max {}/rok)\n- \u{017d}ivotn\u{00ed} poji\u{0161}t\u{011b}n\u{00ed} (max {}/rok)\n- Dary (max 15 % z\u{00e1}kladu dan\u{011b})\n- Odborov\u{00e9} p\u{0159}\u{00ed}sp\u{011b}vky (max {}/rok)",
                    fmt_czk(tc.deduction_cap_mortgage),
                    fmt_czk(tc.deduction_cap_pension),
                    fmt_czk(tc.deduction_cap_life_insurance),
                    fmt_czk(tc.deduction_cap_union_dues),
                )
            } else {
                "Nezdaniteln\u{00e9} \u{010d}\u{00e1}sti z\u{00e1}kladu dan\u{011b} jsou \u{010d}\u{00e1}stky, kter\u{00e9} si ode\u{010d}tete od z\u{00e1}kladu dan\u{011b} P\u{0158}ED v\u{00fd}po\u{010d}tem dan\u{011b} (na rozd\u{00ed}l od slev, kter\u{00e9} se ode\u{010d}\u{00ed}taj\u{00ed} od dan\u{011b} samotn\u{00e9}). Pat\u{0159}\u{00ed} sem \u{00fa}roky z hypot\u{00e9}ky, penzijn\u{00ed} spo\u{0159}en\u{00ed}, \u{017e}ivotn\u{00ed} poji\u{0161}t\u{011b}n\u{00ed}, dary a odborov\u{00e9} p\u{0159}\u{00ed}sp\u{011b}vky. Konkr\u{00e9}tn\u{00ed} stropy z\u{00e1}vis\u{00ed} na zda\u{0148}ovac\u{00ed}m obdob\u{00ed}.".into()
            };
            HelpTopic {
                title: "Odpo\u{010d}ty ze z\u{00e1}kladu dan\u{011b}",
                simple,
                legal: "Nezdaniteln\u{00e9} \u{010d}\u{00e1}sti z\u{00e1}kladu dan\u{011b} upravuje \u{00a7} 15 z\u{00e1}kona \u{010d}. 586/1992 Sb. \u{00da}roky z \u{00fa}v\u{011b}ru na bydlen\u{00ed} (\u{00a7} 15 odst. 3). Penzijn\u{00ed} p\u{0159}ipoji\u{0161}t\u{011b}n\u{00ed}/spo\u{0159}en\u{00ed} (\u{00a7} 15 odst. 5): \u{010d}\u{00e1}stka nad 12 000 K\u{010d}. Soukrom\u{00e9} \u{017e}ivotn\u{00ed} poji\u{0161}t\u{011b}n\u{00ed} (\u{00a7} 15 odst. 6). Dary na ve\u{0159}ejn\u{011b} prosp\u{011b}\u{0161}n\u{00e9} \u{00fa}\u{010d}ely (\u{00a7} 15 odst. 1): min 2 % z\u{00e1}kladu dan\u{011b} nebo 1 000 K\u{010d}, max 15 %. Stropy se m\u{011b}n\u{00ed} podle roku.",
            }
        }
        HelpTopicId::Ztpp => {
            let simple = if let Some(tc) = tc {
                let spouse = fmt_czk(tc.spouse_credit);
                let spouse_double = fmt_czk(Amount::new(tc.spouse_credit.halere() * 2, 0));
                format!(
                    "ZTP/P je pr\u{016f}kaz pro osoby se zvl\u{00e1}\u{0161}t\u{011b} t\u{011b}\u{017e}k\u{00fd}m zdravotn\u{00ed}m posti\u{017e}en\u{00ed}m. V kontextu dan\u{00ed} m\u{00e1} ZTP/P vliv na:\n\n- Sleva na man\u{017e}ela/ku se zdvojn\u{00e1}sobuje (z {spouse} na {spouse_double})\n- Da\u{0148}ov\u{00e9} zv\u{00fd}hodn\u{011b}n\u{00ed} na d\u{00ed}t\u{011b} se zdvojn\u{00e1}sobuje\n\nZTP/P status se prokazuje pr\u{016f}kazem vydan\u{00fd}m \u{00da}\u{0159}adem pr\u{00e1}ce \u{010c}R."
                )
            } else {
                "ZTP/P je pr\u{016f}kaz pro osoby se zvl\u{00e1}\u{0161}t\u{011b} t\u{011b}\u{017e}k\u{00fd}m zdravotn\u{00ed}m posti\u{017e}en\u{00ed}m. V kontextu dan\u{00ed} m\u{00e1} ZTP/P vliv na:\n\n- Sleva na man\u{017e}ela/ku se zdvojn\u{00e1}sobuje\n- Da\u{0148}ov\u{00e9} zv\u{00fd}hodn\u{011b}n\u{00ed} na d\u{00ed}t\u{011b} se zdvojn\u{00e1}sobuje\n\nZTP/P status se prokazuje pr\u{016f}kazem vydan\u{00fd}m \u{00da}\u{0159}adem pr\u{00e1}ce \u{010c}R.".into()
            };
            HelpTopic {
                title: "ZTP/P -- zvl\u{00e1}\u{0161}t\u{011b} t\u{011b}\u{017e}k\u{00e9} posti\u{017e}en\u{00ed} s pr\u{016f}vodcem",
                simple,
                legal: "Dr\u{017e}itel pr\u{016f}kazu ZTP/P je definov\u{00e1}n v \u{00a7} 34 z\u{00e1}kona \u{010d}. 329/2011 Sb. o poskytov\u{00e1}n\u{00ed} d\u{00e1}vek osob\u{00e1}m se zdravotn\u{00ed}m posti\u{017e}en\u{00ed}m. Zdvojn\u{00e1}soben\u{00ed} slevy na man\u{017e}ela/ku: \u{00a7} 35ba odst. 1 p\u{00ed}sm. b) z\u{00e1}kona \u{010d}. 586/1992 Sb. Zdvojn\u{00e1}soben\u{00ed} zv\u{00fd}hodn\u{011b}n\u{00ed} na d\u{00ed}t\u{011b}: \u{00a7} 35c odst. 1 t\u{00e9}ho\u{017e} z\u{00e1}kona.",
            }
        }
        HelpTopicId::SlevaNaManzela => {
            let simple = if let Some(tc) = tc {
                let year = year_from_tc(tc);
                let income_limit = fmt_czk(tc.spouse_income_limit);
                let credit = fmt_czk(tc.spouse_credit);
                let credit_double = fmt_czk(Amount::new(tc.spouse_credit.halere() * 2, 0));
                format!(
                    "Slevu na man\u{017e}ela/ku si m\u{016f}\u{017e}ete uplatnit, pokud v\u{00e1}\u{0161} man\u{017e}el/ka m\u{011b}l/a za zda\u{0148}ovac\u{00ed} obdob\u{00ed} vlastn\u{00ed} ro\u{010d}n\u{00ed} p\u{0159}\u{00ed}jmy nep\u{0159}esahuj\u{00ed}c\u{00ed} {income_limit}. Do t\u{011b}chto p\u{0159}\u{00ed}jm\u{016f} se nezapo\u{010d}\u{00ed}t\u{00e1}vaj\u{00ed} nap\u{0159}. rodi\u{010d}ovsk\u{00fd} p\u{0159}\u{00ed}sp\u{011b}vek, porodn\u{00e9}, d\u{00e1}vky st\u{00e1}tn\u{00ed} soci\u{00e1}ln\u{00ed} podpory \u{010d}i stipendia.\n\nSleva \u{010d}in\u{00ed} {credit} ro\u{010d}n\u{011b}. Pokud je man\u{017e}el/ka dr\u{017e}itelem ZTP/P, sleva se zdvojn\u{00e1}sobuje na {credit_double}. Sleva se proporcionalizuje podle m\u{011b}s\u{00ed}c\u{016f} -- po\u{010d}\u{00ed}t\u{00e1} se od m\u{011b}s\u{00ed}ce, na jeho\u{017e} po\u{010d}\u{00e1}tku byly podm\u{00ed}nky spln\u{011b}ny.\n\nD\u{016f}le\u{017e}it\u{00e9}: slevu na man\u{017e}ela/ku NELZE uplatnit, pokud pou\u{017e}\u{00ed}v\u{00e1}te pau\u{0161}\u{00e1}ln\u{00ed} v\u{00fd}daje. (rok {year})"
                )
            } else {
                "Slevu na man\u{017e}ela/ku si m\u{016f}\u{017e}ete uplatnit, pokud v\u{00e1}\u{0161} man\u{017e}el/ka m\u{011b}l/a za zda\u{0148}ovac\u{00ed} obdob\u{00ed} n\u{00ed}zk\u{00e9} vlastn\u{00ed} ro\u{010d}n\u{00ed} p\u{0159}\u{00ed}jmy. Do t\u{011b}chto p\u{0159}\u{00ed}jm\u{016f} se nezapo\u{010d}\u{00ed}t\u{00e1}vaj\u{00ed} nap\u{0159}. rodi\u{010d}ovsk\u{00fd} p\u{0159}\u{00ed}sp\u{011b}vek, porodn\u{00e9} \u{010d}i stipendia.\n\nKonkr\u{00e9}tn\u{00ed} v\u{00fd}\u{0161}e slevy a limit p\u{0159}\u{00ed}jm\u{016f} z\u{00e1}vis\u{00ed} na zda\u{0148}ovac\u{00ed}m obdob\u{00ed}. Pokud je man\u{017e}el/ka dr\u{017e}itelem ZTP/P, sleva se zdvojn\u{00e1}sobuje. Sleva se proporcionalizuje podle m\u{011b}s\u{00ed}c\u{016f}.\n\nD\u{016f}le\u{017e}it\u{00e9}: slevu na man\u{017e}ela/ku NELZE uplatnit, pokud pou\u{017e}\u{00ed}v\u{00e1}te pau\u{0161}\u{00e1}ln\u{00ed} v\u{00fd}daje.".into()
            };
            HelpTopic {
                title: "Sleva na man\u{017e}ela/ku",
                simple,
                legal: "Sleva na man\u{017e}ela/ku je upravena v \u{00a7} 35ba odst. 1 p\u{00ed}sm. b) z\u{00e1}kona \u{010d}. 586/1992 Sb. Podm\u{00ed}nka: man\u{017e}el/ka \u{017e}ij\u{00ed}c\u{00ed} ve spole\u{010d}n\u{00e9} dom\u{00e1}cnosti s vlastn\u{00ed}m ro\u{010d}n\u{00ed}m p\u{0159}\u{00ed}jmem nep\u{0159}esahuj\u{00ed}c\u{00ed}m z\u{00e1}konem stanoven\u{00fd} limit. Do vlastn\u{00ed}ho p\u{0159}\u{00ed}jmu se nezapo\u{010d}\u{00ed}t\u{00e1}vaj\u{00ed} d\u{00e1}vky dle \u{00a7} 35ba odst. 1 p\u{00ed}sm. b).\n\nU dr\u{017e}itele ZTP/P se sleva zdvojn\u{00e1}sobuje. Proporcionalizace dle \u{00a7} 35ba odst. 3 -- 1/12 za ka\u{017e}d\u{00fd} m\u{011b}s\u{00ed}c, na jeho\u{017e} po\u{010d}\u{00e1}tku byly podm\u{00ed}nky spln\u{011b}ny. P\u{0159}i pau\u{0161}\u{00e1}ln\u{00ed}ch v\u{00fd}daj\u{00ed}ch (\u{00a7} 7 odst. 7) nelze slevu uplatnit (\u{00a7} 35ca).",
            }
        }
    }
}

/// Derive year from TaxYearConstants based on progressive threshold heuristic.
/// The progressive_threshold changes each year, so we use it as a fingerprint.
fn year_from_tc(tc: &TaxYearConstants) -> i32 {
    // Known thresholds per year (from constants.rs)
    let czk = tc.progressive_threshold.to_czk() as i64;
    match czk {
        1_582_812 => 2024,
        1_675_440 => 2025,
        1_762_500 => 2026,
        _ => {
            // Fallback: try to estimate year from threshold
            // progressive_threshold = 48 * average_monthly_salary
            // Average salary grows ~5% per year from 2024 base
            2025
        }
    }
}
