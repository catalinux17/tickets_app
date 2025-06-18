const cities: string[] = ["BARCELONA", "PARIS", "ROME", "BERLIN", "LONDON", "WASHINGTON", "AMSTERDAM"];

const cityLinks: { [key: string]: string[] } = {
    BARCELONA: ["ROME"],
    PARIS: ["ROME"],
    ROME: ["BARCELONA", "PARIS", "BERLIN", "LONDON", "AMSTERDAM"],
    BERLIN: ["ROME", "WASHINGTON"],
    LONDON: ["ROME"],
    AMSTERDAM: ["ROME"],
    WASHINGTON: ["BERLIN"]
};

const routeDetails: {
    station1: string;
    station2: string;
    price: number;
    distance: number;
}[] = [
        {
            station1: "ROME",
            station2: "LONDON",
            price: 470,
            distance: 230,
        },
        {
            station1: "ROME",
            station2: "BERLIN",
            price: 231,
            distance: 535,
        },
        {
            station1: "ROME",
            station2: "AMSTERDAM",
            price: 313,
            distance: 5213,
        },
        {
            station1: "ROME",
            station2: "PARIS",
            price: 213,
            distance: 12311,
        },
        {
            station1: "BERLIN",
            station2: "WASHINGTON",
            price: 2222,
            distance: 23111,
        },
        {
            station1: "ROME",
            station2: "BARCELONA",
            price: 313,
            distance: 2131,
        },
    ];

export function calculatePriceAndDistance(station1: String, station2: String): [number, number] {
    let price: number = 0;
    let distance: number = 0;
    routeDetails.forEach((r) => {
        console.log(r.station1, r.station2, station1, station2)
        console.log(r.price, r.distance)
        if (
            (r.station1 == station1 && r.station2 == station2) ||
            (r.station1 == station2 && r.station2 == station1)
        ) {
            price = r.price
            distance = r.distance
        }
    });
    return [price, distance]
}


export { cities, cityLinks, routeDetails }

