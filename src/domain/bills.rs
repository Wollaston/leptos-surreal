use std::fmt::Display;

use leptos::IntoView;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BillType {
    HR,
    S,
    HJRES,
    SJRES,
    HCONRES,
    SCONRES,
    HRES,
    SRES,
}
impl Display for BillType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use BillType::*;
        match self {
            HR => write!(f, "HR"),
            S => write!(f, "S"),
            HJRES => write!(f, "HJRES"),
            SJRES => write!(f, "SJRES"),
            HCONRES => write!(f, "HCONRES"),
            SCONRES => write!(f, "SCONRES"),
            HRES => write!(f, "HRES"),
            SRES => write!(f, "SRES"),
        }
    }
}

impl IntoView for BillType {
    fn into_view(self) -> leptos::View {
        self.to_string().into_view()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BillTextVersionAbbreviation {
    AS,
    ASH,
    ATH,
    ATS,
    CDH,
    CDS,
    CPH,
    CPS,
    EAH,
    EAS,
    EH,
    ENR,
    ES,
    FPH,
    FPS,
    HDS,
    IH,
    IPH,
    IPS,
    IS,
    LTH,
    LTS,
    PAP,
    PCS,
    PP,
    PVTL,
    PL,
    RCH,
    RCS,
    RDS,
    RFH,
    RFS,
    RH,
    RHUC,
    RIH,
    RS,
    RTH,
    RTS,
    SC,
    STATPVT,
    STAT,
}

impl Display for BillTextVersionAbbreviation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use BillTextVersionAbbreviation::*;
        match self {
            AS => write!(f, "Amendment Ordered to be Printed (Senate)"),
            ASH => write!(f, "Additional Sponsors House"),
            ATH => write!(f, "Agreed to House"),
            ATS => write!(f, "Agreed to Senate"),
            CDH => write!(f, "Committee Discharged House"),
            CDS => write!(f, "Committee Discharged Senate"),
            CPH => write!(f, "Considered and Passed House"),
            CPS => write!(f, "Considered and Passed Senate"),
            EAH => write!(f, "Engrossed Amendment House"),
            EAS => write!(f, "Engrossed Amendment Senate"),
            EH => write!(f, "Engrossed in House"),
            ENR => write!(f, "Enrolled Bill"),
            ES => write!(f, "Engrossed in Senate"),
            FPH => write!(f, "Failed Passage House"),
            FPS => write!(f, "Failed Passage Senate"),
            HDS => write!(f, "Held at Desk Senate"),
            IH => write!(f, "Introduced in House"),
            IPH => write!(f, "Indefinitely Postponed House"),
            IPS => write!(f, "Indefinitely Postponed Senate"),
            IS => write!(f, "Introduced in Senate"),
            LTH => write!(f, "Laid on Table in House"),
            LTS => write!(f, "Laid on Table in Senate"),
            PAP => write!(f, "Printed as Passed"),
            PCS => write!(f, "Placed on Calendar Senate"),
            PP => write!(f, "Public Print"),
            PVTL => write!(f, "Private Law"),
            PL => write!(f, "Public Law"),
            RCH => write!(f, "Reference Change House"),
            RCS => write!(f, "Reference Change Senate"),
            RDS => write!(f, "Received in Senate"),
            RFH => write!(f, "Referred in House"),
            RFS => write!(f, "Referred in Senate"),
            RH => write!(f, "Reported in House"),
            RHUC => write!(f, "Returned to the House by Unanimous Consent"),
            RIH => write!(f, "Referral Instructions House"),
            RS => write!(f, "Reported to Senate"),
            RTH => write!(f, "Referred to Committee House"),
            RTS => write!(f, "Referred to Committee Senate"),
            SC => write!(f, "Sponsor Change"),
            STATPVT => write!(f, "Statutes at Large (Private Law)"),
            STAT => write!(f, "Statute"),
        }
    }
}
