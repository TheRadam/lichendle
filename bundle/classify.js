function classify(name) {
    let genus = document.getElementById("genus").value;
    let species = document.getElementById("species").value;
    console.log(genus);
    console.log(species);
    console.log([genus, species].join(" ").toLowerCase() == name.toLowerCase());

    if ([genus, species].join(" ").toLowerCase() == name.toLowerCase()) {
        document.getElementById("result").innerHTML = "<br><span class='text-emerald-600'>YOU GOT THAT LICHEN!</span>"
    } else {
        document.getElementById("result").innerHTML = "<br><span class='text-red-600'>WRONG!</span>"
    }
}