use super::*;

#[function_component]
pub fn AppHome() -> Html {
    // let show = use_state(|| false);
    // let onclick = {
    //     let counter = counter.clone();
    //     move |_| {
    //         let value = *counter + 1;
    //         counter.set(value);
    //     }
    // };

    html! {
        <>
            <NavHeader />
            <WorkersTable />
            <ServicesTable />
            <SocketsTable />
        </>
    }
}
