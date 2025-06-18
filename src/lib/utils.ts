
function convertDate(date: Date) {
    let d = date;
    return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(
        2,
        "0"
    )}-${String(d.getDate()).padStart(2, "0")}`;
}

export { convertDate }
