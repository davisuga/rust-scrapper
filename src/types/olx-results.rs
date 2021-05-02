use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OlxResults {
    pub ab_test_groups: Option<String>,
    pub device_type: Option<String>,
    pub listing_props: ListingProps,
    pub urls: Option<Urls>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListingProps {
    pub ab_test_groups: String,
    pub ad_list: Vec<AdList>,
    pub base_url: String,
    pub categories: Vec<Category>,
    pub data_context: DataContext,
    pub filter_tab_props: FilterTabProps,
    pub filters: Filters,
    pub has_modal: bool,
    pub listing_header_props: ListingHeaderProps,
    pub locations: Vec<Location>,
    pub mode: String,
    pub next_locations: Vec<NextLocation>,
    pub next_page_link: String,
    pub olx_pay_eligibile: bool,
    pub page_index: i64,
    pub page_limit: i64,
    pub page_size: i64,
    pub page_title: PageTitle,
    pub previous_page_link: ::serde_json::Value,
    pub search_box_props: SearchBoxProps,
    pub search_categories: Vec<SearchCategory>,
    pub searchboxes: Vec<Searchbox>,
    pub selected_next_locations: Vec<::serde_json::Value>,
    pub total_of_ads: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdList {
    pub account_activity_status: Option<AccountActivityStatus>,
    pub category: Option<String>,
    pub date: Option<i64>,
    pub image_count: Option<i64>,
    #[serde(default)]
    pub images: Vec<Image>,
    pub is_chat_enabled: Option<bool>,
    pub is_featured: Option<bool>,
    pub is_visual_highlight: Option<bool>,
    pub is_visual_highlight_test: Option<bool>,
    pub last_bump_age_secs: Option<String>,
    pub list_id: Option<i64>,
    pub location: Option<String>,
    pub old_price: Option<String>,
    pub olx_delivery: Option<OlxDelivery>,
    pub olx_pay: Option<OlxPay>,
    pub position: Option<i64>,
    pub price: Option<String>,
    pub priority_ad_image: Option<bool>,
    pub professional_ad: Option<bool>,
    #[serde(default)]
    pub properties: Vec<Property>,
    pub search_category_level_one: Option<i64>,
    pub search_category_level_zero: Option<i64>,
    pub subject: Option<String>,
    #[serde(default)]
    pub tags: Vec<::serde_json::Value>,
    pub thumbnail: Option<String>,
    pub url: Option<String>,
    pub vehicle_report: Option<VehicleReport>,
    pub video_count: Option<i64>,
    pub is_pub_listing_item: Option<bool>,
    pub pub_type: Option<String>,
    pub gemini_pub: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountActivityStatus {
    pub is_online: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub original: String,
    pub original_alt: String,
    pub thumbnail: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OlxDelivery {
    pub enabled: bool,
    pub weight: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OlxPay {
    pub enabled: bool,
    pub installments: Vec<Installment>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Installment {
    pub fee: Option<Fee>,
    pub highlighted: Option<bool>,
    pub price: Price,
    pub times: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fee {
    pub label: String,
    pub raw: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub label: String,
    pub raw: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    pub label: String,
    pub name: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VehicleReport {
    pub description: ::serde_json::Value,
    pub enabled: bool,
    pub report_link: ::serde_json::Value,
    pub report_title: ::serde_json::Value,
    pub tags: ::serde_json::Value,
    pub title: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub count: i64,
    pub friendly_name: String,
    pub leaf: Option<String>,
    pub level: String,
    pub name: String,
    pub parent: String,
    pub url: String,
    pub value: String,
    #[serde(rename = "breadcrumb_name")]
    pub breadcrumb_name: Option<String>,
    #[serde(rename = "min_location")]
    pub min_location: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataContext {
    pub locations: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterTabProps {
    #[serde(rename = "LocationLink")]
    pub location_link: String,
    #[serde(rename = "LocationText")]
    pub location_text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filters {
    pub keyword: String,
    pub page_index: i64,
    pub region: i64,
    pub selected_ad_types: Vec<String>,
    pub sorting: String,
    pub state: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListingHeaderProps {
    pub ad_types: Vec<AdType>,
    pub available_orders: Vec<AvailableOrder>,
    pub current_ad_type: i64,
    pub current_order: i64,
    pub current_pay_ship: i64,
    #[serde(rename = "olxPayFilterAB")]
    pub olx_pay_filter_ab: bool,
    pub pay_ship_options: Vec<PayShipOption>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdType {
    pub count: i64,
    pub filter_value: Vec<String>,
    pub label: String,
    pub lurker_event: String,
    pub value: i64,
    pub subtitle: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableOrder {
    pub label: String,
    pub lurker_event: String,
    pub value: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayShipOption {
    pub filter_value: i64,
    pub label: String,
    pub lurker_event: String,
    pub value: i64,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub subtitle: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub label: String,
    pub level: String,
    pub url: String,
    pub value: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NextLocation {
    pub label: String,
    pub locations: Vec<Location2>,
    pub rows: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location2 {
    pub count: i64,
    pub label: String,
    pub level: String,
    pub url: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageTitle {
    pub full: String,
    pub simple: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchBoxProps {
    pub black_friday_link: String,
    pub categories: Vec<Category2>,
    pub keyword: String,
    pub mode: String,
    pub search_by_title: bool,
    pub search_links: Vec<SearchLink>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category2 {
    pub count: i64,
    pub friendly_name: String,
    pub leaf: Option<String>,
    pub level: String,
    pub name: String,
    pub parent: String,
    pub url: String,
    pub value: String,
    #[serde(rename = "breadcrumb_name")]
    pub breadcrumb_name: Option<String>,
    #[serde(rename = "min_location")]
    pub min_location: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchLink {
    pub label: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchCategory {
    pub friendly_name: String,
    pub leaf: Option<String>,
    pub level: String,
    pub name: String,
    pub parent: Option<String>,
    #[serde(rename = "special_case")]
    pub special_case: Option<SpecialCase>,
    pub value: String,
    #[serde(rename = "breadcrumb_name")]
    pub breadcrumb_name: Option<String>,
    #[serde(rename = "min_location")]
    pub min_location: Option<String>,
    pub order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpecialCase {
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Searchbox {
    pub label: String,
    pub link: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Urls {
    #[serde(rename = "FAVORITES_API_SECRET")]
    pub favorites_api_secret: String,
    #[serde(rename = "FAVORITES_API_URL")]
    pub favorites_api_url: String,
    #[serde(rename = "REPOSITORIES_API_URL")]
    pub repositories_api_url: String,
    #[serde(rename = "SENTRY_ENVIRONMENT_NAME")]
    pub sentry_environment_name: String,
}
