/**
 * @type {HTMLFormElement}
 */
const signInForm = document.getElementById("signin-form");

fetchUsers.addEventListener("click", async () => {
	console.log("Getting users");
	try {
		let res = await fetch("http://localhost:8080/api/users", {
			method: "GET",
			credentials: "include",
		});
		let data = await res.json();
		if (res.ok) {
			let users = data;
			console.table(users);
			users.forEach((user) => {
				const userItem = document.createElement("li");
				userItem.innerText = user.username;
				usersContainer.appendChild(userItem);
			});
		} else {
			alert(`Error Fetching Users from API: ${data.error}`);
		}
	} catch (error) {
		console.error("Something Went Wrong", error);
	}
});

signInForm.addEventListener("submit", async (ev) => {
	ev.preventDefault();
	const username = ev.target.username.value;
	const password = ev.target.password.value;
	const credentials = { username, password };
	try {
		console.log("Try Login");
		let res = await fetch("http://localhost:8080/api/auth/signin", {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			credentials: "include",
			body: JSON.stringify(credentials),
		});
		let data = await res.json();
		console.log(data);

		if (res.ok) {
			alert("Signed In");
		} else {
			alert(`Error Signing in: ${data.error}`);
		}
	} catch (error) {
		alert("Something Went Wrong");
		console.error(error);
	}
});
