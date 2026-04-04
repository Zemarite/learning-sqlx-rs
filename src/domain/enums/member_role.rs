use crate::domain::errors::{DomainError, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemberRole {
    AccountExecutive,
    AiResearchScientist,
    Ceo,
    DevopsEngineer,
    FinanceAnalyst,
    FrontendEngineer,
    HeadOfSales,
    HrDirector,
    MarketingSpecialist,
    OperationsManager,
    ProductDesigner,
    ProductManager,
    SeniorBackendEngineer,
    SeniorUxDesigner,
}

impl MemberRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            MemberRole::AccountExecutive => "Account Executive",
            MemberRole::AiResearchScientist => "AI Research Scientist",
            MemberRole::Ceo => "CEO",
            MemberRole::DevopsEngineer => "DevOps Engineer",
            MemberRole::FinanceAnalyst => "Finance Analyst",
            MemberRole::FrontendEngineer => "Frontend Engineer",
            MemberRole::HeadOfSales => "Head of Sales",
            MemberRole::HrDirector => "HR Director",
            MemberRole::MarketingSpecialist => "Marketing Specialist",
            MemberRole::OperationsManager => "Operations Manager",
            MemberRole::ProductDesigner => "Product Designer",
            MemberRole::ProductManager => "Product Manager",
            MemberRole::SeniorBackendEngineer => "Senior Backend Engineer",
            MemberRole::SeniorUxDesigner => "Senior UX Designer",
        }
    }

    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "account executive" => Ok(MemberRole::AccountExecutive),
            "ai research scientist" => Ok(MemberRole::AiResearchScientist),
            "ceo" => Ok(MemberRole::Ceo),
            "devops engineer" => Ok(MemberRole::DevopsEngineer),
            "finance analyst" => Ok(MemberRole::FinanceAnalyst),
            "frontend engineer" => Ok(MemberRole::FrontendEngineer),
            "head of sales" => Ok(MemberRole::HeadOfSales),
            "hr director" => Ok(MemberRole::HrDirector),
            "marketing specialist" => Ok(MemberRole::MarketingSpecialist),
            "operations manager" => Ok(MemberRole::OperationsManager),
            "product designer" => Ok(MemberRole::ProductDesigner),
            "product manager" => Ok(MemberRole::ProductManager),
            "senior backend engineer" => Ok(MemberRole::SeniorBackendEngineer),
            "senior ux designer" => Ok(MemberRole::SeniorUxDesigner),
            _ => Err(DomainError::InvalidRole(s.to_string())),
        }
    }
}
