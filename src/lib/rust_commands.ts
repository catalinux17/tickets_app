
import { invoke } from "@tauri-apps/api/tauri";
import { convertDate } from "./utils";

async function getAllTickets(statie1: string, statie2: string): Promise<string[]> {
    let file_content: string = await invoke("get_all_tickets", {
        station1: statie1,
        station2: statie2,
    });
    let tickets = file_content.split("\n");
    tickets = tickets.filter((x) => x.length > 2 && x.indexOf("-") != -1);
    tickets = tickets.slice(-5);
    tickets = tickets.map((x) => x.split(",").slice(3, 4)[0]);
    return tickets
}

async function sendTicket(ticket: string, upStation: string, downStation: string, date: Date, price: number, distance: number) {
    if (
        !isTicketValid(ticket, upStation, downStation, date, price, distance)
    )
        return;

    await invoke("create_ticket", {
        regionType: "INTERJUDETEAN",
        date: convertDate(date),
        station1: upStation,
        station2: downStation,
        price: price,
        distance: distance,
        number: ticket,
    });
}

function isTicketValid(ticket: string, station1: string, station2: string, date: Date, price: number, distance: number): boolean {
    if (ticket.length <= 7 || !station1 || !station2 || !date || price == 0 || distance == 0)
        return;

    let validWithoutDash = /^\s*\d{8,9}\s*$/;
    let validWithDash = /^\s*\d{2,3}-\d{5,6}\s*$/;

    return validWithDash.test(ticket) || validWithoutDash.test(ticket);
}

async function deleteLastTicket(upStation: string, downStation: string) {
    await invoke("remove_last_ticket", {
        station1: upStation,
        station2: downStation,
    });
}


export { getAllTickets, sendTicket, isTicketValid, deleteLastTicket }
