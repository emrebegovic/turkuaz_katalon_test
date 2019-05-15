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

WebUI.callTestCase(findTestCase('cansu/TC_11_Create Account and New Sales Order'), [('imsi') : GlobalVariable.imsi, ('msisdn') : GlobalVariable.msisdn
        , ('custID') : GlobalVariable.custID, ('voiceUsageProductID') : GlobalVariable.voiceUsageProductID, ('voiceFUProductID') : GlobalVariable.voiceFUProductID
        , ('smsUsageProductID') : GlobalVariable.smsUsageProductID, ('dataUsageProductID') : GlobalVariable.dataUsageProductID
        , ('billAcctID') : GlobalVariable.billAcctID, ('subID') : GlobalVariable.subID], FailureHandling.STOP_ON_FAILURE)

responseStatus = WS.sendRequest(findTestObject('BillingAccount/UpdateBillingAccountStatus', [('baStatus') : GlobalVariable.baStatus
            , ('billAcctID') : GlobalVariable.billAcctID]))

WS.sendRequest(findTestObject('BillingAccount/InquireBillingAccount', [('custID') : GlobalVariable.custID, ('billAcctID') : GlobalVariable.billAcctID]))

println((((('\ncustID=   ' + GlobalVariable.custID) + '\nbillAcctID=   ') + GlobalVariable.billAcctID) + '\nStatu= ') + 
    GlobalVariable.baStatus)

