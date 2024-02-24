use leptos::logging::log;
use leptos::*;
use leptos_leaflet::leaflet::{LocationEvent, Map};
use leptos_leaflet::*;
use std::time::Duration;

#[component]
pub fn MapPage() -> impl IntoView {
    let (marker_position, set_marker_position) = create_signal(Position::new(51.49, -0.08));
    let (map, set_map) = create_signal(None::<Map>);

    create_effect(move |_| {
        set_interval_with_handle(
            move || {
                set_marker_position.update(|pos| {
                    pos.lat += 0.001;
                    pos.lng += 0.001;
                });
            },
            Duration::from_secs(1),
        )
        .ok()
    });

    create_effect(move |_| {
        if let Some(map) = map.get() {
            log!("Map context {:?}", map);
        }
    });

    let location_found = move |loc: LocationEvent| {
        log!("hello from {:?}", loc.lat_lng());
    };

    let events = MapEvents::new().location_found(location_found);

    view! {
          <MapContainer style="height: 400px" center=Position::new(51.505, -0.09) zoom=13.0 set_view=true map=set_map locate=true watch=true events>
              <TileLayer url="https://tile.openstreetmap.org/{z}/{x}/{y}.png" attribution="&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"/>
              <Marker position=marker_position >
                  <Popup>
                      <strong>{"A pretty CSS3 popup"}</strong>
                  </Popup>
              </Marker>
              <Marker position=position!(51.5, -0.065) draggable=true >
                  <Popup auto_close=false close_on_click=false>
                      <strong>{"A pretty CSS3 popup"}</strong>
                  </Popup>
              </Marker>
              <Marker position=marker_position draggable=true >
                <Popup auto_close=false close_on_click=false>
                  <strong>{"A moving marker"}</strong>
                </Popup>
              </Marker>
              <Tooltip position=position!(51.5, -0.09) permanent=true direction="top">
                  <strong>{"And a tooltip"}</strong>
              </Tooltip>
              <Polyline positions=positions(&[(51.505, -0.09), (51.51, -0.1), (51.51, -0.12)])/>
              <Polygon color="purple" positions=positions(&[ (51.515, -0.09), (51.52, -0.1), (51.52, -0.12)]) >
                <Tooltip sticky=true direction="top">
                    <strong>{"I'm a polygon"}</strong>
                </Tooltip>
            </Polygon>
            <Circle center=position!(51.505, -0.09) color="blue" radius=200.0>
                <Tooltip sticky=true>{"I'm a circle"}</Tooltip>
            </Circle>
        </MapContainer>
    }
}
