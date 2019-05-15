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
Object intlbillacctid = null
Object name = null
Object DBcustID = null
Object DBintlCustID = null
CustomKeywords.'com.oracle.connection.pkg.oracleConnection.connection'('172.30.10.50', 'PTURKUAZSQA', '1525', 'FCBSADM', 'FCBSADM')
def billAcctTable = CustomKeywords.'com.oracle.connection.pkg.oracleConnection.executeQuery'('SELECT * FROM BILL_ACCT WHERE BILL_ACCT_ID =' + GlobalVariable.billAcctID)
def custTable = CustomKeywords.'com.oracle.connection.pkg.oracleConnection.executeQuery'('SELECT * FROM CUST WHERE EXT_CUST_ID =' + GlobalVariable.custID)

while (billAcctTable.next()) {
    name = billAcctTable.getObject('NAME').toString()

	
	
	
	
    //Object name = resultset.getObject("NAME")
    println('SELECT * FROM BILL_ACCT WHERE NAME = ' + name)
}
if (!(billAcctTable)) {
    println('\n***** B A Ş A R I S I Z *****\n')
} else {
    println('\n***** B A Ş A R I L I *****\n')
}
CustomKeywords.'com.oracle.connection.pkg.oracleConnection.closeDatabaseConnection'()




//InquireCustomer
InquireBillAccount = WS.sendRequest(findTestObject('BillingAccount/InquireBillingAccount', [('billAcctID') : GlobalVariable.billAcctID
            , ('custID') : GlobalVariable.custID]))
String xml3 = InquireBillAccount.responseBodyContent
def responseXML = new XmlParser().parseText(xml3)
println('Response XML: ' + xml3)

def respbillAccttID = responseXML.BillingAccount.BILLING_ACCOUNT_KEY.BILLING_ACCOUNT_ID.text()
def respNAME_ON_INVOICE = responseXML.BillingAccount.BILLING_ACCOUNT_DEFINITION.NAME_ON_INVOICE.text()
def respPARENT_RELATION_ID = responseXML.BillingAccount.BILLING_ACCOUNT_DEFINITION.PARENT_RELATION_ID.text()
def respACCOUNT_TYPE = responseXML.BillingAccount.BILLING_ACCOUNT_DEFINITION.ACCOUNT_TYPE.text()
def respACCOUNT_STATUS = responseXML.BillingAccount.BILLING_ACCOUNT_DEFINITION.ACCOUNT_STATUS.text()
def respPIN_CODE = responseXML.BillingAccount.BILLING_ACCOUNT_DEFINITION.PIN_CODE.text()
def respCOMPANY_DEFINITION_ID = responseXML.BillingAccount.BILLING_ACCOUNT_DEFINITION.COMPANY_DEFINITION_ID.text()
//def respSTART_DATE = responseXML.BillingAccount.BILLING_ACCOUNT_DEFINITION.START_DATE.text()
def respACCOUNT_TAX_INFO = responseXML.BillingAccount.BILLING_ACCOUNT_DEFINITION.ACCOUNT_TAX_INFO.text()
def respSTART_DATE = responseXML.BillingAccount.BILLING_ACCOUNT_DEFINITION.START_DATE.text()
def respBILL_PRESENTATION_ID = responseXML.BillingAccount.BILLING_ACCOUNT_PROFILE.BILL_PRESENTATION_ID.text()
def respCREDIT_LIMIT = responseXML.BillingAccount.BILLING_ACCOUNT_PROFILE.CREDIT_LIMIT.text()
def respCURRENCY_CODE = responseXML.BillingAccount.BILLING_ACCOUNT_PROFILE.CURRENCY_CODE.text()
def respFCBS_BILL_CYCLE_ID = responseXML.BillingAccount.BILLING_ACCOUNT_PROFILE.FCBS_BILL_CYCLE_ID.text()
def respProfileSTART_DATE = responseXML.BillingAccount.BILLING_ACCOUNT_PROFILE.START_DATE.text()
def respCONTACT_TYPE = responseXML.BillingAccount.BILLING_ACCOUNT_CONTACT.CONTACT_TYPE.text()


println(                            'BILLING_ACCOUNT_ID = '        + respbillAccttID+
	                                '\nNAME_ON_INVOICE = '         + respNAME_ON_INVOICE + 
	                                '\nPARENT_RELATION_ID = '      + respPARENT_RELATION_ID + 
                                    '\nACCOUNT_TYPE= '             + respACCOUNT_TYPE + 
								    '\nACCOUNT_STATUS= '           + respACCOUNT_STATUS + 
								    '\nPIN_CODE ='                 + respPIN_CODE + 
								    '\nCOMPANY_DEFINITION_ID ='    + respCOMPANY_DEFINITION_ID + 
							        '\nSTART_DATE='                + respSTART_DATE + 
                                    '\nACCOUNT_TAX_INFO='          + respACCOUNT_TAX_INFO + 
                                    '\nSTART_DATE='                + respSTART_DATE + 
                                    '\nBILL_PRESENTATION_ID ='     +  respBILL_PRESENTATION_ID + 
                                    '\nCREDIT_LIMIT ='             + respCREDIT_LIMIT + 
                                    '\nCURRENCY_CODE ='            + respCURRENCY_CODE + 
                                    '\nFCBS_BILL_CYCLE_ID ='       + respFCBS_BILL_CYCLE_ID + 
                                    '\nProfileSTART_DATE ='        + respProfileSTART_DATE + 
                                    '\nCONTACT_TYPE ='             + respCONTACT_TYPE +"\n\n")




if(respNAME_ON_INVOICE == name) {
	println("\nSAME!" + "\ndb NAME : " + name + "\nResponse NAME : " +respNAME_ON_INVOICE + "\n")
	}
else {
	
	println("NOT SAME!")
}
