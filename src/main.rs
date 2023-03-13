use actix_web::{ get, App, HttpResponse, HttpServer, Responder };

#[get("/")]
async fn hello() -> impl Responder {
 HttpResponse::Ok().content_type("text/html; charset=utf-8").body("
  <html>
   <head>
    <title>
     My SAP UI5
    </title>
    <script id = 'sap-ui-bootstrap' src = 'https://openui5.hana.ondemand.com/resources/sap-ui-core.js' data-sap-ui-theme = 'sap_belize' data-sap-ui-libs = 'sap.m'>
    </script>
    <script>
     try{
      var buttonTime = new sap.m.Button(
       { text : 'Show the time'
       , press: function() {
                 var date = new Date();
                 var time = date.toLocaleTimeString();
           
                 alert(time);
                }//press: function() {
       }
      );//var buttonTime = new sap.m.Button({
        
      buttonTime.placeAt('buttonTime');

     }catch(e){//try{
      alert(e.message);

     }//}catch(e){//try{
    </script>
   </head>
   <body class='sapUiBody'>
    <div id='buttonTime'></div>
   </body>
  </html>
 ")
}//async fn hello() -> impl Responder {

#[actix_web::main]
async fn main() -> std::io::Result<()> {
 match HttpServer::new(|| { App::new().service(hello) }).bind("127.0.0.1:8080") {
  Ok(result) => {
   result.run().await

  }//Ok(result) => {
  
  Err(error) => {
   println!("Error: {:?}", error);

   Err(error)
  }//Err(error) => {
 }//match HttpServer::new(|| { App::new().service(hello) }).bind("127.0.0.1:8080") {
}//async fn main() -> std::io::Result<()> {
