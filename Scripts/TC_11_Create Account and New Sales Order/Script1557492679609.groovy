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

WebUI.callTestCase(findTestCase('TC_09_Create_Cust_BillingAccount_Subsc_OK'), [('custID') : GlobalVariable.custID, ('billAcctID') : GlobalVariable.billAcctID
        , ('subID') : GlobalVariable.subID], FailureHandling.STOP_ON_FAILURE)

GlobalVariable.imsi = CustomKeywords.'com.turkuaz.test.pkg.getTransactions.getIMSI'()

println('IMSI: ' + GlobalVariable.imsi)

GlobalVariable.msisdn = CustomKeywords.'com.turkuaz.test.pkg.getTransactions.getMSISDN'()

GlobalVariable.voiceUsageProductID = CustomKeywords.'com.turkuaz.test.pkg.getTransactions.getRandomID'()

GlobalVariable.voiceFUProductID = CustomKeywords.'com.turkuaz.test.pkg.getTransactions.getRandomID'()

GlobalVariable.smsUsageProductID = CustomKeywords.'com.turkuaz.test.pkg.getTransactions.getRandomID'()

GlobalVariable.dataUsageProductID = CustomKeywords.'com.turkuaz.test.pkg.getTransactions.getRandomID'()

responseNSO = WS.sendRequest(findTestObject('Product/NewSalesOrder', [('imsi') : GlobalVariable.imsi, ('msisdn') : GlobalVariable.msisdn
            , ('custID') : GlobalVariable.custID, ('voiceUsageProductID') : GlobalVariable.voiceUsageProductID, ('voiceFUProductID') : GlobalVariable.voiceFUProductID
            , ('smsUsageProductID') : GlobalVariable.smsUsageProductID, ('dataUsageProductID') : GlobalVariable.dataUsageProductID
            , ('billAcctID') : GlobalVariable.billAcctID, ('subID') : GlobalVariable.subID, ('nsoDate') : GlobalVariable.nsoDate]))

String NSOXML = responseNSO.getResponseText()

println(NSOXML)

println((((((((((((((((('\nimsi   = ' + GlobalVariable.imsi) + '\nmsisdn = ') + GlobalVariable.msisdn) + '\ncustID = ') + 
    GlobalVariable.custID) + '\nvoiceUsageProductID =  ') + GlobalVariable.voiceUsageProductID) + '\nvoiceFUProductID = ') + 
    GlobalVariable.voiceFUProductID) + '\nsmsUsageProductID = ') + GlobalVariable.smsUsageProductID) + '\ndataUsageProductID = ') + 
    GlobalVariable.dataUsageProductID) + '\nbillAcctID = ') + GlobalVariable.billAcctID) + '\nsubID = ') + GlobalVariable.subID)

println((((((((((((((((((((((((((((((('\n\n\n\nSELECT * FROM PROD_CHAR_VAL WHERE VAL=  \'' + GlobalVariable.imsi) + '\';    --imsi') + 
    '\nSELECT * FROM PROD_CHAR_VAL WHERE VAL=\'') + GlobalVariable.msisdn) + '\';   ---msisdn') + '\nSELECT * FROM CUST WHERE EXT_CUST_ID = ') + 
    GlobalVariable.custID) + ';') + '\nSELECT * FROM PROD WHERE PROD_ID = ') + GlobalVariable.voiceUsageProductID) + ';  --voiceUsageProductID ') + 
    '\nSELECT * FROM PROD WHERE PROD_ID = ') + GlobalVariable.voiceFUProductID) + ';  --voiceFUProductID') + '\nSELECT * FROM PROD WHERE PROD_ID = ') + 
    GlobalVariable.smsUsageProductID) + ';  --smsUsageProductID') + '\nSELECT * FROM PROD WHERE PROD_ID = ') + GlobalVariable.dataUsageProductID) + 
    ';  -- dataUsageProductID') + '\nSELECT * FROM BILL_ACCT WHERE BILL_ACCT_ID = ') + GlobalVariable.billAcctID) + ';') + 
    '\nSELECT * FROM SUBSC WHERE EXT_SUBSC_ID = ') + GlobalVariable.subID) + ';\n') + 'SELECT * FROM PROD WHERE INTL_PROD_ID IN(\n') + 
    'SELECT INTL_PROD_ID FROM BILL_ACCT_PROD WHERE INTL_BILL_ACCT_ID IN (\n') + 'SELECT INTL_BILL_ACCT_ID FROM BILL_ACCT WHERE BILL_ACCT_ID=  ') + 
    GlobalVariable.billAcctID) + '));\n\n\n\n')

println()

