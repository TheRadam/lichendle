function classify(name) {
    name = name.trim().toLowerCase();
    let genus = document.getElementById("genus").value.trim().toLowerCase();
    let species = document.getElementById("species").value.trim().toLowerCase();
    let [expectedGenus, expectedSpecies] = name.split(" ");

    setBorder(expectedGenus == genus, document.getElementById("genus"));
    setBorder(expectedSpecies == species, document.getElementById("species"));

    if (expectedGenus == genus && expectedSpecies == species) {
        document.getElementById("result").innerHTML = "<br><span class='text-emerald-600'>YOU GOT THAT LICHEN!</span>"
    } else {
        document.getElementById("result").innerHTML = "<br><span class='text-red-600'>WRONG!</span>"
    }
}

function setBorder(condition, input) {
    if (condition) {
        input.classList.remove('border-red-600');
        input.classList.add('border-emerald-600');
        input.setAttribute("disabled", true)
    } else {
        input.classList.add('border-red-600');
    }
}