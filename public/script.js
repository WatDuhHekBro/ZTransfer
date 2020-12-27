// Warning: Extremely cursed "prototyping" code below, proceed with caution.
const table = document.getElementById("files");
const ws = new WebSocket((() => {
	const url = new URL(window.location.href);
	
	if(!url.protocol.startsWith("http"))
		return "ws://localhost/";
	
	const isSecure = url.protocol === "https:";
	url.protocol = isSecure ? "wss:" : "ws:";
	url.pathname = "/websocket";
	
	return url.toString();
})());

const HANDLER = {
	ADD_FILE(data)
	{
		addFile(data.filename, data.size);
	},
	REMOVE_FILE(data)
	{
		for(let i = i; i < table.length; i++)
		{
			const row = table.rows[i];
			const filename = row.cells[0].innerText;
			
			if(data.filename === filename)
				table.deleteRow(i);
		}
	}
};

ws.onopen = console.log;
ws.onclose = console.warn;
ws.onerror = console.error;
ws.onmessage = message => {
	console.log(message);
	let data;
	
	if(data = tryParse(message))
	{
		const action = HANDLER[data.action];
		action && action(data);
	}
};

/**
 * @param {string} filename - The file's name including its extension
 * @param {number} size - The file's size in bytes
 */
function addFile(filename, size)
{
	const row = document.createElement("tr");
	
	const fn = document.createElement("td");
	fn.innerText = filename;
	row.appendChild(fn);
	
	const sz = document.createElement("td");
	sz.innerText = getSizeString(size);
	row.appendChild(sz);
	
	const ex = document.createElement("td");
	ex.appendChild(createDownloadButton(`/download?file=${encodeURIComponent(filename)}`));
	ex.appendChild(createDeleteButton(filename));
	row.appendChild(ex);
	
	table.appendChild(row);
}

function getSizeString(size)
{
	let factor;
	
	// multiplying/dividing by 1024 can also be applied from << 0xA and >> 0xA respectively.
	if(size >= (factor = Math.pow(1024, 4)))
	{
		const sizeOutput = (size / factor).toFixed(2);
		return `${sizeOutput} TB`;
	}
	else if(size >= (factor = Math.pow(1024, 3)))
	{
		const sizeOutput = (size / factor).toFixed(2);
		return `${sizeOutput} GB`;
	}
	else if(size >= (factor = Math.pow(1024, 2)))
	{
		const sizeOutput = (size / factor).toFixed(2);
		return `${sizeOutput} MB`;
	}
	else if(size >= (factor = 1024))
	{
		const sizeOutput = (size / factor).toFixed(2);
		return `${sizeOutput} KB`;
	}
	else if(size >= (factor = 0))
	{
		const sizeOutput = size === 0 ? 0 : (size / factor).toFixed(2);
		return `${sizeOutput} B`;
	}
	else
		throw new Error("You can't have negative file sizes!");
}

function createDownloadButton(href)
{
	const button = document.createElement("button");
	
	button.onclick = () => {
		const dlink = document.createElement("a");
		dlink.download = href;
		dlink.href = window.URL.createObjectURL(new Blob([contents]));
		dlink.click();
		dlink.remove();
	}
	
	return button;
}

function createDeleteButton(filename)
{
	ws.send(JSON.stringify({
		action: "SEND_DELETE_REQUEST",
		filename
	}));
}

function tryParse(json)
{
	try
	{
		return JSON.parse(json)
	}
	catch
	{
		return false;
	}
}