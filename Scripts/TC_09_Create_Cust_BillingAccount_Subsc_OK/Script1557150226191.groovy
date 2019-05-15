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
import com.kms.katalon.core.logging.KeywordLogger

//Create Customer
GlobalVariable.custID = CustomKeywords.'com.turkuaz.test.pkg.getTransactions.getRandomID'()

responseC = WS.sendRequest(findTestObject('Customer/CreateCustomer', [('custID') : GlobalVariable.custID]))

println(responseC.getResponseText())

String FaultMessageControl = responseC.getResponseText()
KeywordLogger log = new KeywordLogger()
log.logInfo("B A Ş A R I L I")
if (!(FaultMessageControl.contains('<FAULT_CODE>'))) {
	log.logInfo("B A Ş A R I L I")
    println('\n***** B A Ş A R I L I *****\n')
} else {
    println('\n***** B A Ş A R I S I Z *****\n')
	log.logInfo("B A Ş A R I S I Z")
}

//Create Billing Account
GlobalVariable.billAcctID = CustomKeywords.'com.turkuaz.test.pkg.getTransactions.getRandomID'()

responseBA = WS.sendRequest(findTestObject('BillingAccount/CreateBillingAccount', [('billAcctID') : GlobalVariable.billAcctID , ('custID') : GlobalVariable.custID]))

println(responseBA.getResponseText())

String FaultMessageControl2 = responseBA.getResponseText()

if (!(FaultMessageControl2.contains('<FAULT_CODE>'))) {
    println('\n***** B A Ş A R I L I *****\n')
} else {
    println('\n***** B A Ş A R I S I Z *****\n')
}

//Create Subs
GlobalVariable.subID = CustomKeywords.'com.turkuaz.test.pkg.getTransactions.getRandomID'()

responseS = WS.sendRequest(findTestObject('Subscription/CreateSubscription', [('custID') : GlobalVariable.custID, ('subID') : GlobalVariable.subID
            , ('billAcctID') : GlobalVariable.billAcctID]))

println(responseS.getResponseText())

String FaultMessageControl3 = responseS.getResponseText()

if (!(FaultMessageControl3.contains('<FAULT_CODE>'))) {
    println('\n***** B A Ş A R I L I *****\n')
} else {
    println('\n***** B A Ş A R I S I Z *****\n')
}

println((((((((('\nSELECT * FROM CUST WHERE EXT_CUST_ID = ' + GlobalVariable.custID) + ';') + '\nSELECT * FROM BILL_ACCT WHERE BILL_ACCT_ID = ') + 
    GlobalVariable.billAcctID) + ';') + '\nSELECT * FROM SUBSC WHERE EXT_SUBSC_ID =') + GlobalVariable.subID) + ';') + '\n\n\n\n')

