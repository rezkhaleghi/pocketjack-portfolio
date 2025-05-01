// use yew::prelude::*;
// use web_sys::HtmlInputElement;
// use gloo_net::http::Request;
// use wasm_bindgen_futures::spawn_local;
// use gloo_timers::callback::Timeout;
// use shared::flight::{AirportItem, GeneralFlightItem};
// use chrono::{Datelike, Local};
// use shared::utils::{get_today_date, get_one_month_later_date, get_default_airports, format_price,format_number_with_commas,format_date};

// // Main app component
// #[function_component(App)]
// fn app() -> Html {
//     // Flight results state
//     let results = use_state(|| vec![]);
    
//     // From and To input references and values
//     let from_input_ref = use_node_ref();
//     let to_input_ref = use_node_ref();
//     let from_value = use_state(String::new);
//     let to_value = use_state(String::new);
//     let from_iata = use_state(String::new);
//     let to_iata = use_state(String::new);

//     // Autocomplete states
//     let from_airports = use_state(|| get_default_airports());
//     let to_airports = use_state(|| get_default_airports());
//     let show_from_dropdown = use_state(|| false);
//     let show_to_dropdown = use_state(|| false);
    
//     // Display results state
//     let show_results = use_state(|| false);

//     let is_loading = use_state(|| false);
//     let alert_max_price_raw = use_state(|| String::from("")); // Raw numeric value
    
//     // Debounce timers
//     let from_timer = use_state(|| None::<Timeout>);
//     let to_timer = use_state(|| None::<Timeout>);
    
//     // Alert form states
//     let alert_from_iata = use_state(String::new);
//     let alert_to_iata = use_state(String::new);
//     let alert_from_value = use_state(String::new);
//     let alert_to_value = use_state(String::new);
//     let alert_from_date: UseStateHandle<String> = use_state(|| get_today_date());
//     let alert_to_date = use_state(|| get_one_month_later_date());
//     let alert_email = use_state(|| String::from("test@gmail.com"));
//     let alert_max_price = use_state(|| String::from(""));
//     let alert_notif_max_count = use_state(|| String::from("2"));
//     let alert_status_message = use_state(String::new);
//     let show_alert_status = use_state(|| false);
    
//     // Alert form input references
//     let alert_from_input_ref = use_node_ref();
//     let alert_to_input_ref = use_node_ref();
    
//     // Alert form autocomplete states
//     let alert_from_airports = use_state(|| get_default_airports());
//     let alert_to_airports = use_state(|| get_default_airports());
//     let show_alert_from_dropdown = use_state(|| false);
//     let show_alert_to_dropdown = use_state(|| false);
//     let alert_from_timer = use_state(|| None::<Timeout>);
//     let alert_to_timer = use_state(|| None::<Timeout>);

//     // Search for airports with debounce
//     let search_airports = |query: String, airports_state: UseStateHandle<Vec<AirportItem>>, 
//                             show_dropdown: UseStateHandle<bool>, timer_state: UseStateHandle<Option<Timeout>>| {
//         if query.trim().is_empty() {
//             airports_state.set(get_default_airports());
//             show_dropdown.set(true);
//             timer_state.set(None);
//             return;
//         }
        
//         if query.trim().len() < 2 {
//             airports_state.set(get_default_airports());
//             show_dropdown.set(true);
//             timer_state.set(None);
//             return;
//         }
        
//         let airports_clone = airports_state.clone();
//         let show_dropdown_clone = show_dropdown.clone();
//         let query_lower = query.to_lowercase();
        
//         timer_state.set(None);
        
//         let new_timer = Timeout::new(500, move || {
//             let query_clone = query_lower.clone();
//             let airports_clone = airports_clone.clone();
//             let show_dropdown_clone = show_dropdown_clone.clone();
            
//             spawn_local(async move {
//                 let url = format!("http://localhost:9999/api/search-airport?query={}", query_clone);
//                 match Request::get(&url).send().await {
//                     Ok(response) => {
//                         if let Ok(airports) = response.json::<Vec<AirportItem>>().await {
//                             // Log the API response for debugging
//                             web_sys::console::log_1(&format!("API returned {} airports for query '{}'", airports.len(), query_clone).into());
                            
//                             // Filter and limit to top 3 results
//                             let filtered_airports: Vec<AirportItem> = airports
//                                 .into_iter()
//                                 .filter(|airport| {
//                                     airport.iata.to_lowercase().starts_with(&query_clone) ||
//                                     airport.name.to_lowercase().contains(&query_clone)
//                                 })
//                                 .take(3)
//                                 .collect();
                            
//                             if filtered_airports.is_empty() {
//                                 airports_clone.set(Vec::new());
//                             } else {
//                                 airports_clone.set(filtered_airports);
//                             }
//                             show_dropdown_clone.set(true);
//                         } else {
//                             web_sys::console::log_1(&"Failed to parse API response".into());
//                             airports_clone.set(get_default_airports());
//                             show_dropdown_clone.set(true);
//                         }
//                     },
//                     Err(err) => {
//                         web_sys::console::log_1(&format!("API error: {:?}", err).into());
//                         airports_clone.set(get_default_airports());
//                         show_dropdown_clone.set(true);
//                     }
//                 }
//             });
//         });
        
//         timer_state.set(Some(new_timer));
//         show_dropdown.set(false); // Hide dropdown until search results load
//     };

//     // Handle from input change
//     let on_from_change = {
//         let from_value = from_value.clone();
//         let from_airports = from_airports.clone();
//         let show_from_dropdown = show_from_dropdown.clone();
//         let from_timer = from_timer.clone();
        
//         Callback::from(move |e: InputEvent| {
//             let input: HtmlInputElement = e.target_unchecked_into();
//             let value = input.value();
//             from_value.set(value.clone());
            
//             search_airports(
//                 value,
//                 from_airports.clone(),
//                 show_from_dropdown.clone(),
//                 from_timer.clone()
//             );
//         })
//     };

//     // Handle to input change
//     let on_to_change = {
//         let to_value = to_value.clone();
//         let to_airports = to_airports.clone();
//         let show_to_dropdown = show_to_dropdown.clone();
//         let to_timer = to_timer.clone();
        
//         Callback::from(move |e: InputEvent| {
//             let input: HtmlInputElement = e.target_unchecked_into();
//             let value = input.value();
//             to_value.set(value.clone());
            
//             search_airports(
//                 value,
//                 to_airports.clone(),
//                 show_to_dropdown.clone(),
//                 to_timer.clone()
//             );
//         })
//     };

//     // Handle alert from input change
//     let on_alert_from_change = {
//         let alert_from_value = alert_from_value.clone();
//         let alert_from_airports = alert_from_airports.clone();
//         let show_alert_from_dropdown = show_alert_from_dropdown.clone();
//         let alert_from_timer = alert_from_timer.clone();
        
//         Callback::from(move |e: InputEvent| {
//             let input: HtmlInputElement = e.target_unchecked_into();
//             let value = input.value();
//             alert_from_value.set(value.clone());
            
//             search_airports(
//                 value,
//                 alert_from_airports.clone(),
//                 show_alert_from_dropdown.clone(),
//                 alert_from_timer.clone()
//             );
//         })
//     };

//     // Handle alert to input change
//     let on_alert_to_change = {
//         let alert_to_value = alert_to_value.clone();
//         let alert_to_airports = alert_to_airports.clone();
//         let show_alert_to_dropdown = show_alert_to_dropdown.clone();
//         let alert_to_timer = alert_to_timer.clone();
        
//         Callback::from(move |e: InputEvent| {
//             let input: HtmlInputElement = e.target_unchecked_into();
//             let value = input.value();
//             alert_to_value.set(value.clone());
            
//             search_airports(
//                 value,
//                 alert_to_airports.clone(),
//                 show_alert_to_dropdown.clone(),
//                 alert_to_timer.clone()
//             );
//         })
//     };

//     // Handle focus for from input
//     let on_from_focus = {
//         let show_from_dropdown = show_from_dropdown.clone();
//         let from_airports = from_airports.clone();
//         Callback::from(move |_| {
//             from_airports.set(get_default_airports());
//             show_from_dropdown.set(true);
//         })
//     };

//     // Handle focus for to input
//     let on_to_focus = {
//         let show_to_dropdown = show_to_dropdown.clone();
//         let to_airports = to_airports.clone();
//         Callback::from(move |_| {
//             to_airports.set(get_default_airports());
//             show_to_dropdown.set(true);
//         })
//     };

//     // Handle focus for alert from input
//     let on_alert_from_focus = {
//         let show_alert_from_dropdown = show_alert_from_dropdown.clone();
//         let alert_from_airports = alert_from_airports.clone();
//         Callback::from(move |_| {
//             alert_from_airports.set(get_default_airports());
//             show_alert_from_dropdown.set(true);
//         })
//     };

//     // Handle focus for alert to input
//     let on_alert_to_focus = {
//         let show_alert_to_dropdown = show_alert_to_dropdown.clone();
//         let alert_to_airports = alert_to_airports.clone();
//         Callback::from(move |_| {
//             alert_to_airports.set(get_default_airports());
//             show_alert_to_dropdown.set(true);
//         })
//     };

//     // Handle blur for from input
//     let on_from_blur = {
//         let show_from_dropdown = show_from_dropdown.clone();
//         Callback::from(move |_| {
//             let show_from_dropdown = show_from_dropdown.clone();
//             Timeout::new(200, move || {
//                 show_from_dropdown.set(false);
//             }).forget();
//         })
//     };

//     // Handle blur for to input
//     let on_to_blur = {
//         let show_to_dropdown = show_to_dropdown.clone();
//         Callback::from(move |_| {
//             let show_to_dropdown = show_to_dropdown.clone();
//             Timeout::new(200, move || {
//                 show_to_dropdown.set(false);
//             }).forget();
//         })
//     };

//     // Handle blur for alert from input
//     let on_alert_from_blur = {
//         let show_alert_from_dropdown = show_alert_from_dropdown.clone();
//         Callback::from(move |_| {
//             let show_alert_from_dropdown = show_alert_from_dropdown.clone();
//             Timeout::new(200, move || {
//                 show_alert_from_dropdown.set(false);
//             }).forget();
//         })
//     };

//     // Handle blur for alert to input
//     let on_alert_to_blur = {
//         let show_alert_to_dropdown = show_alert_to_dropdown.clone();
//         Callback::from(move |_| {
//             let show_alert_to_dropdown = show_alert_to_dropdown.clone();
//             Timeout::new(200, move || {
//                 show_alert_to_dropdown.set(false);
//             }).forget();
//         })
//     };

//     // In the on_from_select callback, update both the visible value and the stored IATA
//     let on_from_select = {
//         let from_input_ref = from_input_ref.clone();
//         let from_value = from_value.clone();
//         let from_iata = from_iata.clone();
//         let show_from_dropdown = show_from_dropdown.clone();
        
//         Callback::from(move |airport: AirportItem| {
//             if let Some(input) = from_input_ref.cast::<HtmlInputElement>() {
//                 let display_value = format!("{} - {} ({})", airport.iata.to_uppercase(), airport.name, airport.country_code);
//                 input.set_value(&display_value);
//                 from_value.set(display_value);
//                 from_iata.set(airport.iata);
//                 show_from_dropdown.set(false);
//             }
//         })
//     };

//     // Do the same for on_to_select
//     let on_to_select = {
//         let to_input_ref = to_input_ref.clone();
//         let to_value = to_value.clone();
//         let to_iata = to_iata.clone();
//         let show_to_dropdown = show_to_dropdown.clone();
        
//         Callback::from(move |airport: AirportItem| {
//             if let Some(input) = to_input_ref.cast::<HtmlInputElement>() {
//                 let display_value = format!("{} - {} ({})", airport.iata.to_uppercase(), airport.name, airport.country_code);
//                 input.set_value(&display_value);
//                 to_value.set(display_value);
//                 to_iata.set(airport.iata);
//                 show_to_dropdown.set(false);
//             }
//         })
//     };

//     // Alert form from select
//     let on_alert_from_select = {
//         let alert_from_input_ref = alert_from_input_ref.clone();
//         let alert_from_value = alert_from_value.clone();
//         let alert_from_iata = alert_from_iata.clone();
//         let show_alert_from_dropdown = show_alert_from_dropdown.clone();
        
//         Callback::from(move |airport: AirportItem| {
//             if let Some(input) = alert_from_input_ref.cast::<HtmlInputElement>() {
//                 let display_value = format!("{} - {} ({})", airport.iata.to_uppercase(), airport.name, airport.country_code);
//                 input.set_value(&display_value);
//                 alert_from_value.set(display_value);
//                 alert_from_iata.set(airport.iata);
//                 show_alert_from_dropdown.set(false);
//             }
//         })
//     };

//     // Alert form to select
//     let on_alert_to_select = {
//         let alert_to_input_ref = alert_to_input_ref.clone();
//         let alert_to_value = alert_to_value.clone();
//         let alert_to_iata = alert_to_iata.clone();
//         let show_alert_to_dropdown = show_alert_to_dropdown.clone();
        
//         Callback::from(move |airport: AirportItem| {
//             if let Some(input) = alert_to_input_ref.cast::<HtmlInputElement>() {
//                 let display_value = format!("{} - {} ({})", airport.iata.to_uppercase(), airport.name, airport.country_code);
//                 input.set_value(&display_value);
//                 alert_to_value.set(display_value);
//                 alert_to_iata.set(airport.iata);
//                 show_alert_to_dropdown.set(false);
//             }
//         })
//     };

//     // Handle changes for alert form inputs
//     let on_from_date_change = {
//         let alert_from_date = alert_from_date.clone();
//         Callback::from(move |e: Event| {
//             let input: HtmlInputElement = e.target_unchecked_into();
//             alert_from_date.set(input.value());
//         })
//     };

//     let on_to_date_change = {
//         let alert_to_date = alert_to_date.clone();
//         Callback::from(move |e: Event| {
//             let input: HtmlInputElement = e.target_unchecked_into();
//             alert_to_date.set(input.value());
//         })
//     };

//     let on_email_change = {
//         let alert_email = alert_email.clone();
//         Callback::from(move |e: Event| {
//             let input: HtmlInputElement = e.target_unchecked_into();
//             alert_email.set(input.value());
//         })
//     };

//     let on_max_price_change = {
//         let alert_max_price = alert_max_price.clone();
//         let alert_max_price_raw = alert_max_price_raw.clone();
//         Callback::from(move |e: InputEvent| {
//             let input: HtmlInputElement = e.target_unchecked_into();
//             let raw_value = input.value().replace(",", "");
//             alert_max_price_raw.set(raw_value.clone());
//             let formatted_value = format_number_with_commas(&raw_value);
//             alert_max_price.set(formatted_value);
//         })
//     };

//     let on_notif_count_change = {
//         let alert_notif_max_count = alert_notif_max_count.clone();
//         Callback::from(move |e: Event| {
//             let input: HtmlInputElement = e.target_unchecked_into();
//             alert_notif_max_count.set(input.value());
//         })
//     };

//     let on_swap = {
//         let from_value = from_value.clone();
//         let to_value = to_value.clone();
//         let from_iata = from_iata.clone();
//         let to_iata = to_iata.clone();
//         let from_input_ref = from_input_ref.clone();
//         let to_input_ref = to_input_ref.clone();
//         let results = results.clone();
//         let show_results = show_results.clone();

//         Callback::from(move |_| {
//             // Swap display values
//             let temp_value = (*from_value).clone();
//             from_value.set((*to_value).clone());
//             to_value.set(temp_value);

//             // Swap IATA codes
//             let temp_iata = (*from_iata).clone();
//             from_iata.set((*to_iata).clone());
//             to_iata.set(temp_iata);

//             // Update input fields
//             if let Some(from_input) = from_input_ref.cast::<HtmlInputElement>() {
//                 from_input.set_value(&(*from_value));
//             }
//             if let Some(to_input) = to_input_ref.cast::<HtmlInputElement>() {
//                 to_input.set_value(&(*to_value));
//             }

//             // Clear search results
//             results.set(vec![]);
//             show_results.set(false);
//         })
//     };
//     // Handle search for flights form submission
//     let on_search = {
//         let from_iata = from_iata.clone();
//         let to_iata = to_iata.clone();
//         let show_results = show_results.clone();
//         let results = results.clone();
//         let is_loading = is_loading.clone();
        
//         Callback::from(move |e: SubmitEvent| {
//             e.prevent_default();
            
//             if !from_iata.is_empty() && !to_iata.is_empty() {
//                 is_loading.set(true);
//                 let from = (*from_iata).clone();
//                 let to = (*to_iata).clone();
//                 let show_results = show_results.clone();
//                 let results = results.clone();
//                 let is_loading = is_loading.clone();
                
//                 spawn_local(async move {
//                     let payload = serde_json::json!({
//                         "from": from,
//                         "to": to
//                     });
                    
//                     let url = "http://localhost:9999/api/fair-fly";
//                     match Request::post(url)
//                         .header("Content-Type", "application/json")
//                         .json(&payload)
//                         .unwrap()
//                         .send()
//                         .await
//                     {
//                         Ok(response) => {
//                             if response.status() == 200 {
//                                 if let Ok(api_results) = response.json::<Vec<GeneralFlightItem>>().await {
//                                     results.set(api_results);
//                                     show_results.set(true);
//                                 } else {
//                                     results.set(vec![]);
//                                     show_results.set(true);
//                                     web_sys::console::log_1(&"Failed to parse flight results".into());
//                                 }
//                             } else {
//                                 results.set(vec![]);
//                                 show_results.set(true);
//                                 let error_text = response.text().await.unwrap_or_else(|_| "No flights found".to_string());
//                                 web_sys::console::log_1(&format!("API error: {}", error_text).into());
//                             }
//                         },
//                         Err(err) => {
//                             results.set(vec![]);
//                             show_results.set(true);
//                             web_sys::console::log_1(&format!("Network error: {:?}", err).into());
//                         }
//                     }
//                     is_loading.set(false);
//                 });
//             }
//         })
//     };

//     // Handle alert form submission
//     let on_alert_submit = {
//         let alert_from_iata = alert_from_iata.clone();
//         let alert_to_iata = alert_to_iata.clone();
//         let alert_from_date = alert_from_date.clone();
//         let alert_to_date = alert_to_date.clone();
//         let alert_email = alert_email.clone();
//         let alert_max_price_raw = alert_max_price_raw.clone();
//         let alert_notif_max_count = alert_notif_max_count.clone();
//         let alert_status_message = alert_status_message.clone();
//         let show_alert_status = show_alert_status.clone();
        
//         Callback::from(move |e: SubmitEvent| {
//             e.prevent_default();
            
//             if !alert_from_iata.is_empty() && !alert_to_iata.is_empty() {
//                 let from = (*alert_from_iata).clone();
//                 let to = (*alert_to_iata).clone();
//                 let from_date = (*alert_from_date).clone();
//                 let to_date = (*alert_to_date).clone();
//                 let email = (*alert_email).clone();
//                 let max_price = (*alert_max_price_raw).clone().parse::<i64>().unwrap_or(0);
//                 let notif_max_count = (*alert_notif_max_count).clone().parse::<i32>().unwrap_or(1);
                
//                 let alert_status_message = alert_status_message.clone();
//                 let show_alert_status = show_alert_status.clone();
                
//                 spawn_local(async move {
//                     let payload = serde_json::json!({
//                         "from": from,
//                         "to": to,
//                         "from_date": from_date,
//                         "to_date": to_date,
//                         "email": email,
//                         "max_price": max_price,
//                         "notif_max_count": notif_max_count
//                     });
                    
//                     let url = "http://localhost:9999/api/price-alert";
//                     match Request::post(url)
//                         .header("Content-Type", "application/json")
//                         .json(&payload)
//                         .unwrap()
//                         .send()
//                         .await
//                     {
//                         Ok(response) => {
//                             if response.status() == 201 {
//                                 alert_status_message.set("Alert successfully created!".to_string());
//                             } else {
//                                 let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
//                                 alert_status_message.set(format!("Failed to create alert: {}", error_text));
//                             }
//                         },
//                         Err(err) => {
//                             alert_status_message.set(format!("Network error: {:?}", err));
//                         }
//                     }
//                     show_alert_status.set(true);
//                     let show_alert_status = show_alert_status.clone();
//                     Timeout::new(5000, move || {
//                         show_alert_status.set(false);
//                     }).forget();
//                 });
//             } else {
//                 let alert_status_message = alert_status_message.clone();
//                 let show_alert_status = show_alert_status.clone();
                
//                 alert_status_message.set("Please select both departure and destination airports".to_string());
//                 show_alert_status.set(true);
                
//                 let show_alert_status = show_alert_status.clone();
//                 Timeout::new(5000, move || {
//                     show_alert_status.set(false);
//                 }).forget();
//             }
//         })
//     };

//     // Format date function
//     let format_date = |date_str: &str| -> String {
//         format_date(date_str, "FA")  };

//     let current_year = Local::now().year();

//     // Separate domestic and international airports for "From" dropdown
//     let from_domestic_airports: Vec<AirportItem> = from_airports
//         .iter()
//         .filter(|airport| airport.country_code == "IR" && airport.iata != "IKA")
//         .cloned()
//         .collect();
//     let from_international_airports: Vec<AirportItem> = from_airports
//         .iter()
//         .filter(|airport| airport.country_code != "IR" || airport.iata == "IKA")
//         .cloned()
//         .collect();

//     // Separate domestic and international airports for "To" dropdown
//     let to_domestic_airports: Vec<AirportItem> = to_airports
//         .iter()
//         .filter(|airport| airport.country_code == "IR" && airport.iata != "IKA")
//         .cloned()
//         .collect();
//     let to_international_airports: Vec<AirportItem> = to_airports
//         .iter()
//         .filter(|airport| airport.country_code != "IR" || airport.iata == "IKA")
//         .cloned()
//         .collect();

//     // Separate domestic and international airports for "Alert From" dropdown
//     let alert_from_domestic_airports: Vec<AirportItem> = alert_from_airports
//         .iter()
//         .filter(|airport| airport.country_code == "IR" && airport.iata != "IKA")
//         .cloned()
//         .collect();
//     let alert_from_international_airports: Vec<AirportItem> = alert_from_airports
//         .iter()
//         .filter(|airport| airport.country_code != "IR" || airport.iata == "IKA")
//         .cloned()
//         .collect();

//     // Separate domestic and international airports for "Alert To" dropdown
//     let alert_to_domestic_airports: Vec<AirportItem> = alert_to_airports
//         .iter()
//         .filter(|airport| airport.country_code == "IR" && airport.iata != "IKA")
//         .cloned()
//         .collect();
//     let alert_to_international_airports: Vec<AirportItem> = alert_to_airports
//         .iter()
//         .filter(|airport| airport.country_code != "IR" || airport.iata == "IKA")
//         .cloned()
//         .collect();

//     html! {
//         <>
//           <div class="form-header" style="margin-bottom: 1rem; text-align: center; margin-top: 2rem;">
//         <h1>{"🔔 Set a Flight Deal Alert"}</h1>
//         <p style="margin-top: -1rem; margin-bottom: 1rem;">{"Get notified when flight prices drop below your target price."}</p>
//     </div>
//     <div class="search-container">
//         <form class="alert-form" onsubmit={on_alert_submit}>
//             <div class="form-group" style="position: relative;">
//                 <label for="alertFromLocation">{"From"}</label>
//                 <input 
//                     type="text" 
//                     id="alertFromLocation" 
//                     ref={alert_from_input_ref.clone()}
//                     value={(*alert_from_value).clone()}
//                     oninput={on_alert_from_change}
//                     onfocus={on_alert_from_focus}
//                     onblur={on_alert_from_blur}
//                     placeholder="departure" 
//                     required=true 
//                     autocomplete="off"
//                 />
//                 // Alert From Autocomplete dropdown
//                 if *show_alert_from_dropdown && !alert_from_airports.is_empty() {
//                     <div style="position: absolute; width: 300px; left: 0; top: 100%; background: var(--card); border: 1px solid var(--border); border-radius: var(--radius); max-height: 200px; overflow-y: auto; z-index: 100; font-size: 0.9rem;">
//                         // Domestic Airports
//                         if !alert_from_domestic_airports.is_empty() {
//                             <div style="padding: 0.75rem 1rem; background: var(--title); font-weight: bold; border-bottom: 1px solid var(--border);">
//                                 {"Domestic (Iran)"}
//                             </div>
//                         }
//                         {
//                             alert_from_domestic_airports.iter().map(|airport| {
//                                 let airport_clone = airport.clone();
//                                 let on_click = {
//                                     let on_alert_from_select = on_alert_from_select.clone();
//                                     let airport = airport_clone.clone();
//                                     Callback::from(move |_| {
//                                         on_alert_from_select.emit(airport.clone());
//                                     })
//                                 };
//                                 html! {
//                                     <div 
//                                         onclick={on_click}
//                                         style="padding: 0.75rem 1rem; cursor: pointer; border-bottom: 1px solid var(--border);"
//                                         onmouseover={Callback::from(|_| {
//                                             let element = web_sys::window()
//                                                 .and_then(|win| win.document())
//                                                 .and_then(|doc| doc.active_element());
//                                             if let Some(el) = element {
//                                                 el.set_attribute("style", "background-color: var(--card-hover);").ok();
//                                             }
//                                         })}
//                                         onmouseout={Callback::from(|_| {
//                                             let element = web_sys::window()
//                                                 .and_then(|win| win.document())
//                                                 .and_then(|doc| doc.active_element());
//                                             if let Some(el) = element {
//                                                 el.set_attribute("style", "background-color: var(--card);").ok();
//                                             }
//                                         })}
//                                     >
//                                         <strong>{airport.iata.clone()}</strong>{" - "}{airport.name.clone()}{" ("}{airport.country_code.clone()}{")"}
//                                     </div>
//                                 }
//                             }).collect::<Html>()
//                         }
//                         // International Airports
//                         if !alert_from_international_airports.is_empty() {
//                             <div style="padding: 0.75rem 1rem; background: var(--title); font-weight: bold; border-bottom: 1px solid var(--border);">
//                                 {"International"}
//                             </div>
//                         }
//                         {
//                             alert_from_international_airports.iter().map(|airport| {
//                                 let airport_clone = airport.clone();
//                                 let on_click = {
//                                     let on_alert_from_select = on_alert_from_select.clone();
//                                     let airport = airport_clone.clone();
//                                     Callback::from(move |_| {
//                                         on_alert_from_select.emit(airport.clone());
//                                     })
//                                 };
//                                 html! {
//                                     <div 
//                                         onclick={on_click}
//                                         style="padding: 0.75rem 1rem; cursor: pointer; border-bottom: 1px solid var(--border);"
//                                         onmouseover={Callback::from(|_| {
//                                             let element = web_sys::window()
//                                                 .and_then(|win| win.document())
//                                                 .and_then(|doc| doc.active_element());
//                                             if let Some(el) = element {
//                                                 el.set_attribute("style", "background-color: var(--card-hover);").ok();
//                                             }
//                                         })}
//                                         onmouseout={Callback::from(|_| {
//                                             let element = web_sys::window()
//                                                 .and_then(|win| win.document())
//                                                 .and_then(|doc| doc.active_element());
//                                             if let Some(el) = element {
//                                                 el.set_attribute("style", "background-color: var(--card);").ok();
//                                             }
//                                         })}
//                                     >
//                                         <strong>{airport.iata.clone()}</strong>{" - "}{airport.name.clone()}{" ("}{airport.country_code.clone()}{")"}
//                                     </div>
//                                 }
//                             }).collect::<Html>()
//                         }
//                     </div>
//                 }
//             </div>
            
//             <div class="form-group" style="position: relative;">
//                 <label for="alertToLocation">{"To"}</label>
//                 <input 
//                     type="text" 
//                     id="alertToLocation" 
//                     ref={alert_to_input_ref.clone()}
//                     value={(*alert_to_value).clone()}
//                     oninput={on_alert_to_change}
//                     onfocus={on_alert_to_focus}
//                     onblur={on_alert_to_blur}
//                     placeholder="destination" 
//                     required=true 
//                     autocomplete="off"
//                 />
//                 // Alert To Autocomplete dropdown
//                 if *show_alert_to_dropdown && !alert_to_airports.is_empty() {
//                     <div style="position: absolute; width: 300px; left: 0; top: 100%; background: var(--card); border: 1px solid var(--border); border-radius: var(--radius); max-height: 200px; overflow-y: auto; z-index: 100; font-size: 0.9rem;">
//                         // Domestic Airports
//                         if !alert_to_domestic_airports.is_empty() {
//                             <div style="padding: 0.75rem 1rem; background: var(--title); font-weight: bold; border-bottom: 1px solid var(--border);">
//                                 {"Domestic (Iran)"}
//                             </div>
//                         }
//                         {
//                             alert_to_domestic_airports.iter().map(|airport| {
//                                 let airport_clone = airport.clone();
//                                 let on_click = {
//                                     let on_alert_to_select = on_alert_to_select.clone();
//                                     let airport = airport_clone.clone();
//                                     Callback::from(move |_| {
//                                         on_alert_to_select.emit(airport.clone());
//                                     })
//                                 };
//                                 html! {
//                                     <div 
//                                         onclick={on_click}
//                                         style="padding: 0.75rem 1rem; cursor: pointer; border-bottom: 1px solid var(--border);"
//                                         onmouseover={Callback::from(|_| {
//                                             let element = web_sys::window()
//                                                 .and_then(|win| win.document())
//                                                 .and_then(|doc| doc.active_element());
//                                             if let Some(el) = element {
//                                                 el.set_attribute("style", "background-color: var(--card-hover);").ok();
//                                             }
//                                         })}
//                                         onmouseout={Callback::from(|_| {
//                                             let element = web_sys::window()
//                                                 .and_then(|win| win.document())
//                                                 .and_then(|doc| doc.active_element());
//                                             if let Some(el) = element {
//                                                 el.set_attribute("style", "background-color: var(--card);").ok();
//                                             }
//                                         })}
//                                     >
//                                         <strong>{airport.iata.clone()}</strong>{" - "}{airport.name.clone()}{" ("}{airport.country_code.clone()}{")"}
//                                     </div>
//                                 }
//                             }).collect::<Html>()
//                         }
//                         // International Airports
//                         if !alert_to_international_airports.is_empty() {
//                             <div style="padding: 0.75rem 1rem; background: var(--title); font-weight: bold; border-bottom: 1px solid var(--border);">
//                                 {"International"}
//                             </div>
//                         }
//                         {
//                             alert_to_international_airports.iter().map(|airport| {
//                                 let airport_clone = airport.clone();
//                                 let on_click = {
//                                     let on_alert_to_select = on_alert_to_select.clone();
//                                     let airport = airport_clone.clone();
//                                     Callback::from(move |_| {
//                                         on_alert_to_select.emit(airport.clone());
//                                     })
//                                 };
//                                 html! {
//                                     <div 
//                                         onclick={on_click}
//                                         style="padding: 0.75rem 1rem; cursor: pointer; border-bottom: 1px solid var(--border);"
//                                         onmouseover={Callback::from(|_| {
//                                             let element = web_sys::window()
//                                                 .and_then(|win| win.document())
//                                                 .and_then(|doc| doc.active_element());
//                                             if let Some(el) = element {
//                                                 el.set_attribute("style", "background-color: var(--card-hover);").ok();
//                                             }
//                                         })}
//                                         onmouseout={Callback::from(|_| {
//                                             let element = web_sys::window()
//                                                 .and_then(|win| win.document())
//                                                 .and_then(|doc| doc.active_element());
//                                             if let Some(el) = element {
//                                                 el.set_attribute("style", "background-color: var(--card);").ok();
//                                             }
//                                         })}
//                                     >
//                                         <strong>{airport.iata.clone()}</strong>{" - "}{airport.name.clone()}{" ("}{airport.country_code.clone()}{")"}
//                                     </div>
//                                 }
//                             }).collect::<Html>()
//                         }
//                     </div>
//                 }
//             </div>

//             <div class="form-group">
//                 <label for="fromDate">{"From Date"}</label>
//                 <input 
//                     type="date" 
//                     id="fromDate" 
//                     value={(*alert_from_date).clone()}
//                     onchange={on_from_date_change}
//                     required=true 
//                 />
//             </div>

//             <div class="form-group">
//                 <label for="toDate">{"To Date"}</label>
//                 <input 
//                     type="date" 
//                     id="toDate" 
//                     value={(*alert_to_date).clone()}
//                     onchange={on_to_date_change}
//                     required=true 
//                 />
//             </div>

//             <div class="form-group">
//                 <label for="notifCount">{"Max Alert"}</label>
//                 <input 
//                     type="number" 
//                     id="notifCount" 
//                     value={(*alert_notif_max_count).clone()}
//                     onchange={on_notif_count_change}
//                     min="1" 
//                     max="10"
//                     required=true
//                     style="-webkit-appearance: none; -moz-appearance: textfield; margin: 0;"
//                 />
//             </div>

//             <div class="form-group">
//                 <label for="maxPrice">{"Max Price ﷼"}</label>
//                 <input 
//                     type="text"
//                     id="maxPrice" 
//                     placeholder="25,500,000" 
//                     value={(*alert_max_price).clone()}
//                     oninput={on_max_price_change}
//                     min="0"
//                     required=true
//                     style="-webkit-appearance: none; -moz-appearance: textfield; margin: 0;"
//                 />
//             </div>
            
//             <div class="form-group">
//                 <label for="email">{"Email Address"}</label>
//                 <input 
//                     type="email" 
//                     id="email" 
//                     onchange={on_email_change}
//                     placeholder="Email" 
//                     required=true 
//                 />
//             </div>
            
//             <button type="submit" disabled={*is_loading} style="margin-top:1.8rem;">
//                 {
//                     if *is_loading {
//                         "Setting..."
//                     } else {
//                         "Set Alert"
//                     }
//                 }
//             </button>
            
//             // Status message
//             if *show_alert_status {
//                 <div class={classes!(
//                     "alert-status",
//                     if alert_status_message.contains("success") { "success" } else { "error" }
//                 )}>
//                     {(*alert_status_message).clone()}
//                 </div>
//             }
//         </form>
//     </div>

//    // Flight Search Form Section
//     <div class="form-header" style="margin-bottom: 1rem; text-align: center;">
//         <h1>{"✈️ Find the Best Flight Deals"}</h1>
//         <p style="margin-top: -1rem; margin-bottom: 1rem; max-width: 600px; margin-left: auto; margin-right: auto;">
//             {"Looking for the best deal? Find the cheapest flights across multiple providers, fast"}
//         </p>
//     </div>
//     <div class="search-container">
//         <form class="search-form" onsubmit={on_search}>
//             <div class="form-group" style="position: relative;">
//                 <label for="fromLocation">{"From"}</label>
//                 <input 
//                     type="text" 
//                     id="fromLocation" 
//                     ref={from_input_ref.clone()}
//                     value={(*from_value).clone()}
//                     oninput={on_from_change}
//                     onfocus={on_from_focus}
//                     onblur={on_from_blur}
//                     placeholder="Enter departure location" 
//                     required=true 
//                     autocomplete="off"
//                 />
//                 // From Autocomplete dropdown
//                 if *show_from_dropdown && !from_airports.is_empty() {
//                     <div style="position: absolute; width: 100%; background: var(--card); border: 1px solid var(--border); border-radius: var(--radius); max-height: 200px; overflow-y: auto; z-index: 10;">
//                         // Domestic Airports
//                         if !from_domestic_airports.is_empty() {
//                             <div style="padding: 0.5rem 1rem; background: var(--title); font-weight: bold; border-bottom: 1px solid var(--border);">
//                                 {"Domestic (Iran)"}
//                             </div>
//                         }
//                         {
//                             from_domestic_airports.iter().map(|airport| {
//                                 let airport_clone = airport.clone();
//                                 let on_click = {
//                                     let on_from_select = on_from_select.clone();
//                                     let airport = airport_clone.clone();
//                                     Callback::from(move |_| {
//                                         on_from_select.emit(airport.clone());
//                                     })
//                                 };
//                                 html! {
//                                     <div 
//                                         onclick={on_click}
//                                         style="padding: 0.5rem 1rem; cursor: pointer; border-bottom: 1px solid var(--border);"
//                                         onmouseover={Callback::from(|_| {
//                                             let element = web_sys::window()
//                                                 .and_then(|win| win.document())
//                                                 .and_then(|doc| doc.active_element());
//                                             if let Some(el) = element {
//                                                 el.set_attribute("style", "background-color: var(--card-hover);").ok();
//                                             }
//                                         })}
//                                         onmouseout={Callback::from(|_| {
//                                             let element = web_sys::window()
//                                                 .and_then(|win| win.document())
//                                                 .and_then(|doc| doc.active_element());
//                                             if let Some(el) = element {
//                                                 el.set_attribute("style", "background-color: var(--card);").ok();
//                                             }
//                                         })}
//                                     >
//                                         <strong>{airport.iata.clone()}</strong>{" - "}{airport.name.clone()}{" ("}{airport.country_code.clone()}{")"}
//                                     </div>
//                                 }
//                             }).collect::<Html>()
//                         }
//                         // International Airports
//                         if !from_international_airports.is_empty() {
//                             <div style="padding: 0.5rem 1rem; background: var(--title); font-weight: bold; border-bottom: 1px solid var(--border);">
//                                 {"International"}
//                             </div>
//                         }
//                         {
//                             from_international_airports.iter().map(|airport| {
//                                 let airport_clone = airport.clone();
//                                 let on_click = {
//                                     let on_from_select = on_from_select.clone();
//                                     let airport = airport_clone.clone();
//                                     Callback::from(move |_| {
//                                         on_from_select.emit(airport.clone());
//                                     })
//                                 };
//                                 html! {
//                                     <div 
//                                         onclick={on_click}
//                                         style="padding: 0.5rem 1rem; cursor: pointer; border-bottom: 1px solid var(--border);"
//                                         onmouseover={Callback::from(|_| {
//                                             let element = web_sys::window()
//                                                 .and_then(|win| win.document())
//                                                 .and_then(|doc| doc.active_element());
//                                             if let Some(el) = element {
//                                                 el.set_attribute("style", "background-color: var(--card-hover);").ok();
//                                             }
//                                         })}
//                                         onmouseout={Callback::from(|_| {
//                                             let element = web_sys::window()
//                                                 .and_then(|win| win.document())
//                                                 .and_then(|doc| doc.active_element());
//                                             if let Some(el) = element {
//                                                 el.set_attribute("style", "background-color: var(--card);").ok();
//                                             }
//                                         })}
//                                     >
//                                         <strong>{airport.iata.clone()}</strong>{" - "}{airport.name.clone()}{" ("}{airport.country_code.clone()}{")"}
//                                     </div>
//                                 }
//                             }).collect::<Html>()
//                         }
//                     </div>
//                 }
//             </div>

//             // Swipe Button
//             <button 
//                 type="button" 
//                 class="swap-button"
//                 onclick={on_swap.clone()}
//                 aria-label="Swap departure and destination"
//             >
//                 <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="var(--foreground)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
//                     <path d="M17 3h4v4"></path>
//                     <path d="M21 7L7 21"></path>
//                     <path d="M7 21H3v-4"></path>
//                 </svg>
//             </button>

//             <div class="form-group" style="position: relative;">
//                 <label for="toLocation">{"To"}</label>
//                 <input 
//                     type="text" 
//                     id="toLocation" 
//                     ref={to_input_ref.clone()}
//                     value={(*to_value).clone()}
//                     oninput={on_to_change}
//                     onfocus={on_to_focus}
//                     onblur={on_to_blur}
//                     placeholder="Enter destination" 
//                     required=true 
//                     autocomplete="off"
//                 />
//                 // To Autocomplete dropdown
//                 if *show_to_dropdown && !to_airports.is_empty() {
//                     <div style="position: absolute; width: 100%; background: var(--card); border: 1px solid var(--border); border-radius: var(--radius); max-height: 200px; overflow-y: auto; z-index: 10;">
//                         // Domestic Airports
//                         if !to_domestic_airports.is_empty() {
//                             <div style="padding: 0.5rem 1rem; background: var(--title); font-weight: bold; border-bottom: 1px solid var(--border);">
//                                 {"Domestic (Iran)"}
//                             </div>
//                         }
//                         {
//                             to_domestic_airports.iter().map(|airport| {
//                                 let airport_clone = airport.clone();
//                                 let on_click = {
//                                     let on_to_select = on_to_select.clone();
//                                     let airport = airport_clone.clone();
//                                     Callback::from(move |_| {
//                                         on_to_select.emit(airport.clone());
//                                     })
//                                 };
//                                 html! {
//                                     <div 
//                                         onclick={on_click}
//                                         style="padding: 0.5rem 1rem; cursor: pointer; border-bottom: 1px solid var(--border);"
//                                         onmouseover={Callback::from(|_| {
//                                             let element = web_sys::window()
//                                                 .and_then(|win| win.document())
//                                                 .and_then(|doc| doc.active_element());
//                                             if let Some(el) = element {
//                                                 el.set_attribute("style", "background-color: var(--card-hover);").ok();
//                                             }
//                                         })}
//                                         onmouseout={Callback::from(|_| {
//                                             let element = web_sys::window()
//                                                 .and_then(|win| win.document())
//                                                 .and_then(|doc| doc.active_element());
//                                             if let Some(el) = element {
//                                                 el.set_attribute("style", "background-color: var(--card);").ok();
//                                             }
//                                         })}
//                                     >
//                                         <strong>{airport.iata.clone()}</strong>{" - "}{airport.name.clone()}{" ("}{airport.country_code.clone()}{")"}
//                                     </div>
//                                 }
//                             }).collect::<Html>()
//                         }
//                         // International Airports
//                         if !to_international_airports.is_empty() {
//                             <div style="padding: 0.5rem 1rem; background: var(--title); font-weight: bold; border-bottom: 1px solid var(--border);">
//                                 {"International"}
//                             </div>
//                         }
//                         {
//                             to_international_airports.iter().map(|airport| {
//                                 let airport_clone = airport.clone();
//                                 let on_click = {
//                                     let on_to_select = on_to_select.clone();
//                                     let airport = airport_clone.clone();
//                                     Callback::from(move |_| {
//                                         on_to_select.emit(airport.clone());
//                                     })
//                                 };
//                                 html! {
//                                     <div 
//                                         onclick={on_click}
//                                         style="padding: 0.5rem 1rem; cursor: pointer; border-bottom: 1px solid var(--border);"
//                                         onmouseover={Callback::from(|_| {
//                                             let element = web_sys::window()
//                                                 .and_then(|win| win.document())
//                                                 .and_then(|doc| doc.active_element());
//                                             if let Some(el) = element {
//                                                 el.set_attribute("style", "background-color: var(--card-hover);").ok();
//                                             }
//                                         })}
//                                         onmouseout={Callback::from(|_| {
//                                             let element = web_sys::window()
//                                                 .and_then(|win| win.document())
//                                                 .and_then(|doc| doc.active_element());
//                                             if let Some(el) = element {
//                                                 el.set_attribute("style", "background-color: var(--card);").ok();
//                                             }
//                                         })}
//                                     >
//                                         <strong>{airport.iata.clone()}</strong>{" - "}{airport.name.clone()}{" ("}{airport.country_code.clone()}{")"}
//                                     </div>
//                                 }
//                             }).collect::<Html>()
//                         }
//                     </div>
//                 }
//             </div>
            
//             <button type="submit" disabled={*is_loading} style="margin-top:1.8rem;">
//                 {
//                     if *is_loading {
//                         "Searching..."
//                     } else {
//                         "Search"
//                     }
//                 }
//             </button>
//         </form>
//     </div>
    
//     <div class={classes!(
//         "results-container",
//         if *show_results { "visible" } else { "" }
//     )}>
//         {
//             if results.is_empty() && *show_results {
//                 html! {
//                     <div style="text-align: center; padding: 1rem; color: var(--text);">
//                         {"No flights found for the selected route."}
//                     </div>
//                 }
//             } else {
//                 html! {
//                     <table>
//                         <thead>
//                             <tr>
//                                 <th>{"Provider"}</th>
//                                 <th>{"Price"}</th>
//                                 <th>{"Date"}</th>
//                                 <th>{"From"}</th>
//                                 <th>{"To"}</th>
//                             </tr>
//                         </thead>
//                         <tbody>
//                             {results.iter().map(|result| {
//                                 let row_class = if result.is_best_period_price {
//                                     "best-period-price"
//                                 } else if result.is_best_day_price {
//                                     "best-day-price"
//                                 } else {
//                                     ""
//                                 };
//                                 html! {
//                                     <tr class={row_class}>
//                                         <td>
//                                             <a 
//                                                 href={if result.provider.starts_with("http://") || result.provider.starts_with("https://") {
//                                                     result.provider.clone()
//                                                 } else {
//                                                     format!("http://{}", result.provider)
//                                                 }}
//                                                 target="_blank" 
//                                                 rel="noopener noreferrer" 
//                                                 style="color: var(--link-color); text-decoration: none; cursor: pointer;"
//                                             >
//                                                 {&result.provider}
//                                             </a>
//                                         </td>
//                                         <td class="price">{format_price(result.price)}</td>
//                                         <td>{format_date(&result.date)}</td>
//                                         <td>{(*from_value).clone()}</td>
//                                         <td>{(*to_value).clone()}</td>
//                                     </tr>
//                                 }
//                             }).collect::<Html>()
//                             }
//                         </tbody>
//                     </table>
//                 }
//             }
//         }
//     </div>

//     // Loading spinner
//     if *is_loading {
//         <div class="loading-spinner">
//             <div class="spinner"></div>
//         </div>
//     }

//     <footer style="margin-top: 3rem; padding: 1rem 0; text-align: center; border-top: 1px solid var(--border); display: flex; flex-direction: column; align-items: center; gap: 0.75rem;">
//         <div style="display: flex; align-items: center; gap: 1rem;">
//             <span style="font-weight: 500;">{"by PocketJack"}</span>
//             <a href="https://github.com/rezkhaleghi" target="_blank" rel="noopener noreferrer" style="color: var(--text); display: flex; align-items: center;">
//                 <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
//                     <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path>
//                 </svg>
//             </a>
//             <a href="mailto:rezaxkhaleghi@gmail.com" style="color: var(--text); display: flex; align-items: center;">
//                 <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
//                     <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"></path>
//                     <polyline points="22,6 12,13 2,6"></polyline>
//                 </svg>
//             </a>
//         </div>
//         <div style="font-size: 0.9rem; color: var(--muted-text, #777);">
//             <span>{format!("© {} False Foundation. All rights reserved.", current_year)}</span>
//         </div>
//     </footer>
// </>
//     }
// }

// fn main() {
//     yew::Renderer::<App>::new().render();
// }