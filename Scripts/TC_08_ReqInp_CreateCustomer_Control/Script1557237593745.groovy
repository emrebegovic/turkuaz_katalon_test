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

response = WS.sendRequest(findTestObject('Customer/CreateCustomer', [('custID') : GlobalVariable.custID]))

String respXml = response.responseBodyContent

println('Response XML: ' + respXml)

def respDataValue = new XmlParser().parseText(respXml)

def transactionID = respDataValue.RequestResponse.TRANSACTION_ID.text()

println(('TRANSACTION_ID = ' + transactionID) + '\n\n')

println(('SELECT * FROM CUST WHERE EXT_CUST_ID = ' + GlobalVariable.custID) + '\n\n')

CustomKeywords.'com.oracle.connection.pkg.oracleConnection.connection'('172.30.10.50', 'PTURKUAZSQA', '1525', 'FCBSADM', 
    'FCBSADM')

def reqInpTable = CustomKeywords.'com.oracle.connection.pkg.oracleConnection.executeQuery'((('SELECT * FROM REQ_INP WHERE TXN_ID =' + 
    '\'') + transactionID) + '\'')

Object intlTxnID = null

while (reqInpTable.next()) {
    intlTxnID = reqInpTable.getObject('INTL_TXN_ID').toString()

    println('SELECT * FROM REQ_INP WHERE INTL_TXN_ID = ' + intlTxnID)
}

if (!(intlTxnID)) {
    println('intl txn id bulunamadı!')
} else {
    println('SELECT * FROM REQ_INP WHERE INTL_TXN_ID = ' + intlTxnID)
}

if (!(reqInpTable)) {
    println('\n***** B A Ş A R I S I Z *****\n')
} else {
    println('\n***** B A Ş A R I L I *****\n')
}

CustomKeywords.'com.oracle.connection.pkg.oracleConnection.closeDatabaseConnection'()

