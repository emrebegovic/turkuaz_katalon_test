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

response2 = WS.sendRequest(findTestObject('Customer/CreateCustomer'))

GlobalVariable.billAcctID = CustomKeywords.'com.turkuaz.test.pkg.getTransactions.getRandomID'()

response1 = WS.sendRequest(findTestObject('BillingAccount/CreateBillingAccount_Manually_Billcycle2_ParentRelationID1', [('custID') : GlobalVariable.custID
            , ('billAcctID') : GlobalVariable.billAcctID, ('billcycle2') : GlobalVariable.billcycle2]))

println('billAcctID : ' + GlobalVariable.billAcctID)

println(response1.getResponseText())

println(response2.getResponseText())

String FaultMessageControl = response1.getResponseText()



if (!(FaultMessageControl.contains('<FAULT_CODE>'))) {
    println('\n***** B A Ş A R I L I *****\n')
} else {
    println('\n***** B A Ş A R I S I Z *****\n')
}

