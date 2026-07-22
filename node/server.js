require("http").createServer((_, response) => response.end("Paasta Node example")).listen(process.env.PORT || 3000, "0.0.0.0");
