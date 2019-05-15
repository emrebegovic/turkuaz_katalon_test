import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import groovy.util.XmlSlurper
import groovy.util.XmlParser;
import groovy.util.slurpersupport.NodeChild;
import groovy.util.XmlNodePrinter;
import groovy.json.JsonSlurper;
import com.eviware.soapui.support.XmlHolder;

//get randomID
GlobalVariable.custID = CustomKeywords.'com.turkuaz.test.pkg.getTransactions.getRandomID'()
GlobalVariable.billAcctID = CustomKeywords.'com.turkuaz.test.pkg.getTransactions.getRandomID'()
println(((('\nSELECT * FROM CUST WHERE EXT_CUST_ID = ' + GlobalVariable.custID) + '\nSELECT * FROM BILL_ACCT WHERE BILL_ACCT_ID = ') + 
    GlobalVariable.billAcctID) + '\n\n\n\n')


//send request 
response = WS.sendRequest(findTestObject('Customer/CreateCompositeCustomer', [('custID') : GlobalVariable.custID, ('billAcctID') : GlobalVariable.billAcctID]))
println(response.getResponseText())

//Error message
String FaultMessageControl = response.getResponseText()
if (!(FaultMessageControl.contains('<FAULT_CODE>'))) {
    println('\n***** B A Ş A R I L I *****\n')
} else {
    println('\n***** B A Ş A R I S I Z *****\n')
}




//Database Connection and get data
CustomKeywords.'com.oracle.connection.pkg.oracleConnection.connection'('172.30.10.50', 'PTURKUAZSQA', '1525', 'FCBSADM', 
    'FCBSADM')
def billAcctTable = CustomKeywords.'com.oracle.connection.pkg.oracleConnection.executeQuery'('SELECT * FROM BILL_ACCT WHERE BILL_ACCT_ID =' + 
    GlobalVariable.billAcctID)
def custTable = CustomKeywords.'com.oracle.connection.pkg.oracleConnection.executeQuery'('SELECT * FROM CUST WHERE EXT_CUST_ID =' +
	GlobalVariable.custID)


Object intlbillacctid =null
Object DBcustID =null
Object DBintlCustID =null

while (billAcctTable.next()) {
    intlbillacctid = billAcctTable.getObject('INTL_BILL_ACCT_ID')

    //Object name = resultset.getObject("NAME")
    println('SELECT * FROM BILL_ACCT WHERE INTL_BILL_ACCT_ID= ' + intlbillacctid)
}

while (custTable.next()) {
	DBcustID = custTable.getObject('EXT_CUST_ID').toString()
	DBintlCustID = custTable.getObject('INTL_CUST_ID').toString()
	//Object name = resultset.getObject("NAME")
	println('SELECT * FROM CUST WHERE EXT_CUST_ID= ' + DBcustID)
}



if (!(billAcctTable)) {
    println('\n***** B A Ş A R I S I Z *****\n')
} else {
    println('\n***** B A Ş A R I L I *****\n')
}
CustomKeywords.'com.oracle.connection.pkg.oracleConnection.closeDatabaseConnection'()








//InquireCustomer
InquireCust = WS.sendRequest(findTestObject('Customer/InquireCustomer', [('custID') : GlobalVariable.custID]))
String xml3 =InquireCust.responseBodyContent
def responseXML = new XmlParser().parseText(xml3)
println("Response XML: " + xml3)
def customerID =responseXML.Customer.CUSTOMER_KEY.CUSTOMER_ID.text()
def fcbsCustomerID =responseXML.Customer.CUSTOMER_KEY.FCBS_CUSTOMER_ID.text()


println("customerID= "+ customerID +"\n" +
	    "fcbsCustomerID= "+ fcbsCustomerID +"\n")

println( " ext cust id= " + DBcustID  + "\nintl cust id= "  + DBintlCustID + "\nfcbs cust id= " + fcbsCustomerID)

if(DBintlCustID == fcbsCustomerID) { 
	println("SAME!")
	}
else {
	
	println("NOT SAME!")
}

