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

println('\nSELECT * FROM CUST WHERE EXT_CUST_ID = ' + GlobalVariable.custID +
	'\nSELECT * FROM BILL_ACCT WHERE BILL_ACCT_ID = ' + GlobalVariable.billAcctID + '\n\n\n\n' )

response = WS.sendRequest(findTestObject('Customer/CreateCompositeCustomer', [('custID') : GlobalVariable.custID, ('billAcctID') : GlobalVariable.billAcctID]))

println(response.getResponseText())

String FaultMessageControl =response.getResponseText()
if (!FaultMessageControl.contains('<FAULT_CODE>')) {
    println('\n***** B A Ş A R I L I *****\n')
} else {
    println('\n***** B A Ş A R I S I Z *****\n')
}

