use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OlxPost {
    pub ad: Ad,
    pub urls: Urls,
    pub ab_test_groups: String,
    pub device_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ad {
    pub list_id: i64,
    pub body: String,
    pub subject: String,
    pub price_label: String,
    pub price_value: String,
    pub old_price: ::serde_json::Value,
    pub professional_ad: bool,
    pub category: i64,
    pub parent_category_name: String,
    pub category_name: String,
    pub search_category_level_zero: i64,
    pub search_category_level_one: i64,
    pub search_category_level_two: i64,
    pub orig_list_time: i64,
    pub ad_reply: String,
    pub friendly_url: String,
    pub loan_specific_data: ::serde_json::Value,
    pub user: User,
    pub phone: Phone,
    pub images: Vec<Image>,
    pub videos: Vec<::serde_json::Value>,
    pub location: Location,
    pub properties: Vec<Property>,
    pub pub_specific_data: Vec<PubSpecificDaum>,
    pub tracking_specific_data: Vec<TrackingSpecificDaum>,
    pub searchboxes: Vec<Searchbox>,
    pub breadcrumb_urls: Vec<BreadcrumbUrl>,
    pub tags: ::serde_json::Value,
    pub car_specific_data: ::serde_json::Value,
    pub real_estate_specific_data: RealEstateSpecificData,
    pub olx_pay: ::serde_json::Value,
    pub olx_delivery: ::serde_json::Value,
    pub vehicle_report: VehicleReport,
    pub description: String,
    pub price: String,
    pub list_time: String,
    pub location_properties: Vec<LocationProperty>,
    pub security_tips: Vec<String>,
    pub slots_id: Vec<String>,
    pub denounce_link: String,
    pub native_vas: Vec<::serde_json::Value>,
    pub is_featured: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub user_id: i64,
    pub user_uid: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Phone {
    pub phone: String,
    pub phone_hidden: bool,
    pub phone_verified: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub original: String,
    pub original_alt: String,
    pub thumbnail: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub address: ::serde_json::Value,
    pub neighbourhood: String,
    pub neighbourhood_id: i64,
    pub municipality: String,
    pub municipality_id: i64,
    pub zipcode: String,
    pub map_lati: i64,
    pub map_long: i64,
    pub uf: String,
    pub ddd: String,
    pub zone_id: i64,
    pub zone: String,
    pub region: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    pub name: String,
    pub label: String,
    pub value: String,
    pub values: ::serde_json::Value,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PubSpecificDaum {
    pub context: String,
    pub data: Vec<Daum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daum {
    pub key: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingSpecificDaum {
    pub key: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Searchbox {
    pub label: String,
    pub link: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BreadcrumbUrl {
    pub label: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RealEstateSpecificData {
    pub show_simulator_button: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VehicleReport {
    pub enabled: bool,
    pub title: ::serde_json::Value,
    pub description: ::serde_json::Value,
    pub report_link: ::serde_json::Value,
    pub report_title: ::serde_json::Value,
    pub tags: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationProperty {
    pub label: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Urls {
    #[serde(rename = "CHAT_URL")]
    pub chat_url: String,
    #[serde(rename = "CARTEIRO_SERVICE_URL")]
    pub carteiro_service_url: String,
    #[serde(rename = "POINTS_OF_SALE_URL")]
    pub points_of_sale_url: String,
    #[serde(rename = "AUTH_URL")]
    pub auth_url: String,
    #[serde(rename = "AUTH_ACCESS_KEY")]
    pub auth_access_key: String,
    #[serde(rename = "GOLLUM_URL")]
    pub gollum_url: String,
    #[serde(rename = "FAVORITES_API_URL")]
    pub favorites_api_url: String,
    #[serde(rename = "FAVORITES_API_SECRET")]
    pub favorites_api_secret: String,
    #[serde(rename = "MY_ACCOUNTS_API_URL")]
    pub my_accounts_api_url: String,
    #[serde(rename = "MY_ACCOUNTS_API_SECRET")]
    pub my_accounts_api_secret: String,
    #[serde(rename = "PHONE_RANGER_API_URL")]
    pub phone_ranger_api_url: String,
    #[serde(rename = "DELIVERY_API_URL")]
    pub delivery_api_url: String,
    #[serde(rename = "SENTRY_ENVIRONMENT_NAME")]
    pub sentry_environment_name: String,
}
